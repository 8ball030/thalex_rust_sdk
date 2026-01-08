# Thalex Rust SDK — Performance Addendum (6 Point Optimizations + Audit Review)

**Context:** The project is used as closed-source software, focus only on performance.  
**Constraint:** SDK/public API refactoring is not possible — optimizations are allowed only inside `src/` (and if necessary, minimal changes to `Cargo.toml` for dependencies).

---

## 0) What I Checked in `src/` Code

The critical path in the current implementation is in `src/ws_client.rs`:

- `run_single_connection()` → reading `ws.next()` → `handle_incoming(...)`
- `handle_incoming()` does **full `serde_json::from_str::<Value>` for every message**
- then:
  - RPC path: `pending_requests.lock().await` → `remove(id)` → `tx.send(text)`
  - Sub path: `subscriptions.lock().await` → `get_mut(channel)` → `sender.send(text)` → cleanup

There are also two clearly unnecessary copies:
- `handle_incoming(text.to_string(), ...)` when `Message::Text(text)` (since `text` is already `String`)
- `String::from_utf8(bin.to_vec())` when `Message::Binary(bin)` (copies the buffer)

---

## 1) Detailed Review of Files from `audit/`

Below is essentially a "code review" of Cursor's conclusions: where I **agree**, where a **correction/clarification** is needed, and what **I would add**.

### 1.1 `audit/FINAL_REPORT.md` — General Conclusion

**I agree with the theses:**
- "Mutex in hot path" — correct: `pending_requests.lock()` and `subscriptions.lock()` are called for every incoming message (at least for messages with `id` or `channel_name`).
- "JSON is fully parsed for every message" — correct: we always parse into `serde_json::Value`, even if we then use only 1–2 fields.
- "No batching for re-subscriptions" — correct: on reconnect, N subscribe messages are sent for N channels.

**Clarification on "concurrent degradation is linear — critical":**
- in real runtime **incoming messages are read by one loop** (`ws.next()` + `handle_incoming()`), i.e., processing is essentially sequential.
- degradation "by number of channels" in the benchmark appears stronger due to artificial task competition, however contention still exists between:
  - the incoming message processing stream
  - calls to `subscribe/unsubscribe/call_rpc`, which also take these same mutexes.

In summary: the conclusion about the bottleneck is correct, but the degree of "linear catastrophe" needs to be interpreted through the real load profile (frequency of subscriptions/unsubscriptions/parallel RPCs).

---

### 1.2 `audit/benchmark_results_analysis.md` — Correctness of Number Interpretation

**Strong part of the report:** order-of-magnitude comparisons are useful:
- RPC path ~300ns looks plausible (simple routing + very short critical section).
- Ticker parse ~2.2µs is also typical for `serde_json::Value` on a relatively small message.

**Critical correction to "fast key check = contains(\"id\":\")":**
- in your code (and in normal JSON-RPC) the `id` field is a **number**, i.e., in the string it's usually `"id":123`, not `"id":"123"`.
- the check `text.contains(r#""id":"#)` / `find(r#""id":"#)` from the report may **not work at all** on real messages.
- correct marker at minimum: `\"id\":` (considering possible spaces/formatting, better to search for `"id"` and colon).

**Risk of false positives:**
- `contains()` may find `"id":` inside nested objects (e.g., in notification payload), which will send the message to the "RPC branch", although this is not an RPC response.
- this is fixed "two-stage": fast check → then light "envelope parsing" (see optimization #4 below), which will verify that `id` is truly top-level.

---

### 1.3 `audit/optimization_recommendations.md` — What's Good and What Should Be Corrected

**Good/realistic:**
- idea to replace `Mutex<HashMap>` with `DashMap` **for subscriptions** (read-heavy) — generally applicable without changing the public API.
- batching re-subscriptions — gives noticeable benefit on reconnect and reduces load.
- backoff/jitter — useful (though this is more about reliability/UX than "nanoseconds").

**What needs to be fixed:**
1) `find(r#""id":"#)` — see above: almost certainly incorrect marker for numeric id.
2) re-subscription batching example in the report holds `subs = subscriptions.lock().await;` and then does `ws.send(...).await` — this is holding the mutex during I/O.
   - in your current `src/` it's exactly the same (lock is held during `.await`).
   - correct approach: **take snapshot of keys under lock**, release lock, then send.
3) suggestion "change `handle_incoming(text: &str)`" — useful, but carefully: in real code you sometimes transfer ownership of `text` to `tx.send(text)`/`sender.send(text)`. Need a strategy "clone only on the path where it's actually needed".

---

### 1.4 `audit/implementation_examples.md` — Applicability

The file is useful as a draft, but it should be perceived as "ideas", not as a ready patch:
- some examples rely on `DashMap` + `.get_mut()` and working with `RefMut`; this requires careful use to not hold the guard during a "long" operation (even though `send()` is not `await`, it's still time under shard-lock).
- some examples don't account for "hold lock across await" in resubscribe.

---

## 2) Additional Report: 6 Point Optimizations (Without SDK Refactoring)

Below are **exactly 6** changes that can be made *inside `src/`*, without changing the external API.

### Optimization 1 — Remove Unnecessary Copies of Incoming Messages (Cheapest Win)

**Current location:**
- `Message::Text(text)` → `handle_incoming(text.to_string(), ...)`
- `Message::Binary(bin)` → `String::from_utf8(bin.to_vec())`

**Why it matters:** these are unnecessary allocations/copies on every message (ticker can be very frequent).

**What to do:**
- pass `text` directly (it's already `String`)
- `String::from_utf8(bin)` without `to_vec()`

**Expected effect:** reduction in alloc/copies (often visible immediately in profile).

---

### Optimization 2 — Don't Hold `subscriptions` Lock During `.send()` (Reduce Mutex Hold Time)

**Current location:** in `handle_incoming()` sub-branch holds `subs.lock().await` until end of processing.

**What to do:**
1) Under lock take `sender.clone()` (UnboundedSender clones cheaply).
2) Drop lock.
3) `sender.send(text)` outside lock.
4) If send failed → briefly take lock and remove entry (optionally: "remove if still same sender").

**Why:** this reduces contention between:
- incoming handler
- `subscribe/unsubscribe`, which also want the lock.

---

### Optimization 3 — For RPC Path: `remove(id)` Under Lock, but `tx.send(text)` Outside Lock

Similar to optimization 2, but for `pending_requests`:

**Current:** lock is held while you do `tx.send(...)`.

**What to do:**
- under lock do `let tx = pending.remove(&id);`
- drop lock
- `tx.send(text)` outside lock

**Why:** reduces delay for "all other" operations that want the pending lock (including `call_rpc`).

---

### Optimization 4 — Replace "parse into Value always" with Light Envelope Parsing

**Goal:** remove heavy construction of `serde_json::Value`, when only `id`/`channel_name` are needed.

**Best compromise without dependency:** small struct:

```rust
#[derive(Deserialize)]
struct Envelope<'a> {
    id: Option<u64>,
    #[serde(borrow)]
    channel_name: Option<std::borrow::Cow<'a, str>>,
}
```

**Pattern:**
- first fast check (`memchr`/`contains`) to filter out "completely uninteresting" messages
- then `serde_json::from_str::<Envelope>(&text)`
- if `id.is_some()` → RPC path
- else if `channel_name.is_some()` → sub path
- else warn/ignore

**Why better than `Value`:**
- fewer allocations
- less CPU for building the tree
- less pressure on allocator

**Important correction to audit:** should search for `"id":` (numeric), not `""id":"`, and better confirm via `Envelope` afterwards.

---

### Optimization 5 — Resubscribe: Snapshot Keys Under Lock + Batching (and Don't Hold Lock During Await)

**Current:**
- hold `subscriptions.lock().await`
- in loop do `ws.send(...).await?` (I/O under lock)

**What to do:**
1) Under lock collect `Vec<String>` of channels (snapshot).
2) Drop lock.
3) Send either:
   - one batched subscribe (if server accepts `channels: [...]`)
   - or fallback: loop without lock

**Plus:** reduction in lock hold time + faster reconnect.

---

### Optimization 6 — Pre-allocate HashMap Capacity (and Cheap Housekeeping)

This is a "minor thing", but it doesn't break the API and often pays off in long-running processes:

- `pending_requests`: can use `HashMap::with_capacity(N)` if there's an expected upper bound of parallel RPCs (e.g., 1024).
- `subscriptions`: `HashMap::with_capacity(M)` (e.g., 128/256).
- when `subscribe()` if channel already exists — don't silently overwrite sender (otherwise "double subscription" leaves old task alive). Can:
  - return Err/log warning and replace carefully, or
  - remove old sender and let it finish.

This is more about **performance and resource control** than "nanoseconds".

---

## 3) Implementation Prioritization (Minimum Risk → Maximum Profit)

1) Opt.1 (copies) — safe and fast.
2) Opt.5 (resubscribe snapshot + batching) — safe, reduces lock+await problem.
3) Opt.2/3 (send outside lock) — also safe, usually gives noticeable effect under contention.
4) Opt.4 (Envelope instead of Value) — medium risk (needs test coverage), but big win.
5) Opt.6 (capacity + housekeeping) — low risk.

---

## 4) What I Would Ask to Run After Changes (Within Existing `benches/`)

To make "before/after" comparison fair:
- `handle_incoming` bench: separately for RPC and subscription path, with real JSON format `id` as number.
- JSON parsing bench: add `Envelope` parsing alongside `Value`.
- Throughput bench: maintain realistic model (one reader-loop) + separate stress test on parallel `subscribe/unsubscribe/call_rpc`.

---

## 5) Summary

I generally **agree** with the audit conclusions (mutex contention + full JSON parse + no batching).  
But some specific "fast checks" in the reports need to be **corrected for real JSON format** (`id` as number) and must remove "lock held across await" in resubscribe — this is one of the most practical problems in current `src/`.

If needed, I can make a second addendum document with **point patch-diffs** for `src/ws_client.rs` (without changing public API), so they can be applied manually.

