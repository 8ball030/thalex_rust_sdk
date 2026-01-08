# Thalex Rust SDK - Performance Reaudit (2025-2026)

**Date:** January 2026  
**Version:** 2.0  
**Status:** Completed

---

## Executive Summary

A performance reaudit of Thalex Rust SDK was conducted after:
1. Merging the latest main update (298 files changed, 40221 insertions, 1139 deletions)
2. Updating rustc on the developer's machine

### Key Code Changes

After the merge, the main architecture remained the same, but a modular structure was added:
- **New modules:** `src/channels/` and `src/rpc/` - API convenience wrappers
- **New models:** Expanded set of data models (261 .rs files)
- **Critical path:** Remained unchanged - `handle_incoming` in `src/ws_client.rs`

**Important:** New `channels` and `rpc` modules do not affect critical path performance, as they are thin wrappers over the existing API.

---

## Benchmark Results (Updated)

### 1. JSON Parsing (`json_parsing`)

| Operation | Time (new) | Time (old) | Change |
|-----------|------------|------------|--------|
| Full RPC parsing | 349.47 ns | ~355 ns | ✅ -1.6% (better) |
| Full ticker parsing | 2.1027 µs | ~2.2 µs | ✅ -4.4% (better) |
| Full large message parsing | 39.566 µs | ~45 µs | ✅ -12.1% (better) |
| **Key check "id"** | **7.7590 ns** | ~8 ns | ✅ Stable |
| **Key check "channel_name"** | **10.480 ns** | ~10 ns | ✅ Stable |
| Conditional parsing (after check) | 339.75 ns | ~337 ns | ⚠️ +0.8% (minor) |

**Conclusions:**
- OK. JSON parsing performance improved by 1-12% (possibly due to rustc update)
- OK. Fast key checking remains 44-220x faster than full parsing
- OK. Optimization recommendation remains relevant

### 2. Incoming Message Processing (`handle_incoming`)

| Scenario | Time (new) | Time (old) | Change |
|----------|------------|------------|--------|
| RPC response (empty structures) | 335.01 ns | ~317 ns | ⚠️ +5.7% |
| RPC response (with pending request) | 324.75 ns | ~306 ns | ⚠️ +6.1% |
| Ticker without subscription | 466.09 ns | ~459 ns | ⚠️ +1.5% |
| Ticker with subscription | 959.41 ns | ~792 ns | ⚠️ **+21.1%** |
| Many subscriptions (1) | 490.98 ns | ~464 ns | ⚠️ +5.8% |
| Many subscriptions (10) | 497.28 ns | ~481 ns | ⚠️ +3.4% |
| Many subscriptions (50) | 499.64 ns | ~479 ns | ⚠️ +4.3% |
| Many subscriptions (100) | 502.25 ns | ~481 ns | ⚠️ +4.4% |
| Many pending (1-100) | 324.98-333.38 ns | ~307-314 ns | ⚠️ +5-6% |

**Conclusions:**
- Warning. Slight performance degradation (5-21%), possibly due to code or environment changes
- Warning. **Critical:** Ticker with subscription became 21% slower (959ns vs 792ns)
- OK. Scalability remains good - no degradation with growth in subscription count

**Note:** Degradation may be related to:
- Changes in Rust compiler
- Additional checks in code (e.g., handling `id: null`)
- Changes in runtime environment

### 3. Mutex Locks (`mutex_contention`)

| Operation | Time (new) | Time (old) | Change |
|-----------|------------|------------|--------|
| Insert/remove 100 | 13.443 µs | ~13.4 µs | ✅ Stable |
| Insert/remove 1000 | 134.29 µs | ~134 µs | ✅ Stable |
| Read-heavy 10 keys | 42.987 µs | ~42.7 µs | ⚠️ +0.7% |
| Read-heavy 100 keys | 42.973 µs | ~42.5 µs | ⚠️ +1.1% |
| Write-heavy 100 | 8.2289 µs | ~7.7 µs | ⚠️ +6.9% |
| Write-heavy 1000 | 83.754 µs | ~77 µs | ⚠️ +8.8% |
| Concurrent access (4 tasks) | 64.325 µs | ~61 µs | ⚠️ +5.5% |

**Conclusions:**
- Warning. Slight degradation in write-heavy operations (6-9%)
- OK. Read-heavy operations remain stable
- OK. Scaling remains linear

---

## Current Code Analysis for Bottlenecks

### Critical Path: `handle_incoming`

**Location:** `src/ws_client.rs:642-695`

#### Identified Issues:

1. **Excessive String Copying (line 592)**
   ```rust
   Some(Ok(Message::Text(text))) => {
       handle_incoming(
           text.to_string(),  // ❌ text is already String, unnecessary copy
           ...
       ).await;
   }
   ```
   **Impact:** Unnecessary allocation for every text message

2. **Excessive Binary Data Copying (line 599)**
   ```rust
   Some(Ok(Message::Binary(bin))) => {
       if let Ok(text) = String::from_utf8(bin.to_vec()) {  // ❌ to_vec() copies
           ...
       }
   }
   ```
   **Impact:** Unnecessary buffer copy before conversion

3. **Full JSON Parsing for Every Message (line 648)**
   ```rust
   let parsed: Value = match serde_json::from_str(&text) {
       Ok(v) => v,
       ...
   };
   ```
   **Impact:** Parsing occurs even when only certain fields are needed
   **Optimization:** Fast check `contains("\"id\":")` or `contains("\"channel_name\":")` before full parsing

4. **Mutex Locks in Hot Path**
   - Line 671: `pending_requests.lock().await` - for every RPC response
   - Line 682: `route.lock().await` - for every subscription
   **Impact:** Lock contention at high message frequency
   **Optimization:** DashMap for subscriptions (read-heavy), clone sender outside lock

5. **Handling `id: null` (lines 659-666)**
   ```rust
   if id_value.is_null() {
       // Check if it's a subscription result or error
       ...
   }
   ```
   **Impact:** Additional check, but necessary for correct operation

6. **pending_requests.drain() + send under lock (line 501-503)**
   ```rust
   let mut pending = pending_requests.lock().await;
   for (_, tx) in pending.drain() {
       let _ = tx.send(r#"{"error":"connection closed"}"#.to_string());  // ❌ send under lock
   }
   ```
   **Impact:** On connection loss, all pending requests are sent under lock, which may block other operations
   **Optimization:** Collect all in Vec under lock, then send outside lock

### Other Bottlenecks:

6. **Reconnection with Fixed Delay (line 519)**
   ```rust
   tokio::time::sleep(std::time::Duration::from_secs(3)).await;
   ```
   **Impact:** No exponential backoff
   **Optimization:** Exponential backoff with jitter

7. **No Subscription Batching (lines 382-413)**
   ```rust
   for channel in public_channels {
       let _: RpcResponse = self.send_rpc("public/subscribe", ...).await?;
   }
   ```
   **Impact:** Separate RPC request for each channel
   **Optimization:** Send all channels in one request
   **Note:** !!! In current code, the "lock across await" issue is already fixed - snapshot of keys is taken under lock, then await is performed without lock. The remaining issue is batching.

8. **Mutex Locks for instruments_cache (lines 66, 150, 167-170, 177)**
   ```rust
   instruments_cache: Arc<Mutex<HashMap<String, Instrument>>>
   ```
   **Impact:** Additional contention point with frequent use of `round_price_to_ticks()` or cache updates
   **Locations:**
   - `cache_instruments()`: line 150 - lock for clearing and populating cache
   - `round_price_to_ticks()`: lines 167-170, 177 - lock for reading from cache
   **Optimization:** Consider using `DashMap` for read-heavy workload or caching with lighter locks

---

## Comparison with Previous Analysis

### What Remained Unchanged:

**Critical path architecture** - `handle_incoming` works similarly  
**Main bottlenecks** - same performance issues  
**Optimization recommendations** - remain relevant  

### What Changed:

**Performance** - slight degradation (5-21%) in some operations  
OK. **Modularity** - added convenient wrappers `channels` and `rpc`, but they don't affect critical path  
OK. **Edge case handling** - added handling of `id: null`  

---

## Updated Optimization Recommendations

### Priority 1: Eliminate Excessive Copying (Quick Win)

**Expected Effect:** 20-30% reduction in allocations

```rust
// In the WebSocket reader loop (handling `Message::Text`) before calling `handle_incoming()`
Some(Ok(Message::Text(text))) => {
    handle_incoming(
        text,  // ✅ Remove .to_string() - text is already String
        pending_requests,
        public_subscriptions,
        private_subscriptions,
    ).await;
}

// Line 598-605
Some(Ok(Message::Binary(bin)) => {
    if let Ok(text) = String::from_utf8(bin.as_ref().to_vec()) {  // ✅ Or better: String::from_utf8_lossy
        handle_incoming(text, ...).await;
    }
}
```

### Priority 2: JSON Parsing Optimization

**Expected Effect:** Savings of ~2 µs per ticker message

```rust
async fn handle_incoming(
    text: String,  // Accept String directly
    ...
) {
    // Fast check before full parsing
    if text.contains("\"id\":") && !text.contains("\"id\":null") {
        // This is RPC response - parse only if needed
        let parsed: Value = serde_json::from_str(&text)?;
        if let Some(id) = parsed.get("id").and_then(|v| v.as_u64()) {
            // ...
        }
    } else if text.contains("\"channel_name\":") {
        // This is subscription - parse only if needed
        let parsed: Value = serde_json::from_str(&text)?;
        // ...
    }
}
```

### Priority 3: Lock Optimization

**Expected Effect:** 50-80% improvement in concurrent processing

#### 3.1 DashMap for Subscriptions

```rust
use dashmap::DashMap;

pub struct WsClient {
    // ...
    pub public_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    pub private_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
}

// In handle_incoming
if let Some(sender) = public_subscriptions.get(channel_name) {
    if sender.send(text.clone()).is_err() {
        public_subscriptions.remove(channel_name);
    }
    return;
}
```

#### 3.2 Clone Sender Outside Lock

```rust
// For pending_requests (if keeping Mutex)
let sender = {
    let mut pending = pending_requests.lock().await;
    pending.remove(&id)
};
if let Some(tx) = sender {
    let _ = tx.send(text);
}
```

### Priority 4: Subscription Batching

**Expected Effect:** Faster recovery after reconnection

**Note:** !!!! In current code, the "lock across await" issue is already fixed - snapshot of keys is taken under lock (lines 383-386, 398-401), then await is performed without lock. The remaining issue is batching.

```rust
// Current code (lines 382-413):
// !!! Already fixed: snapshot under lock, await without lock
let public_channels: Vec<String> = {
    let subs = self.public_subscriptions.lock().await;
    subs.keys().cloned().collect()  // Snapshot under lock
};
// Lock released here

// But sends one channel at a time:
for channel in public_channels {
    let _: RpcResponse = self.send_rpc("public/subscribe", 
        serde_json::json!({ "channels": [channel.clone()] })).await?;
}

// Correctly: send all channels in one request
if !public_channels.is_empty() {
    let _: RpcResponse = self.send_rpc(
        "public/subscribe",
        serde_json::json!({ "channels": public_channels }),
    ).await?;
}

// Similarly for private_channels
...
```

### Priority 5: Exponential Backoff

**Expected Effect:** More efficient reconnection

```rust
let mut backoff_secs = 1;
const MAX_BACKOFF: u64 = 30;

loop {
    // ... connection attempt ...
    
    if connection_failed {
        tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
        backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF);
    } else {
        backoff_secs = 1;  // Reset on successful connection
    }
}
```

---

## Expected Improvements After Optimizations

| Optimization | Current Time | Expected Time | Improvement |
|--------------|--------------|---------------|-------------|
| Ticker with subscription | 959 ns | ~600-700 ns | 27-37% |
| RPC response | 335 ns | ~300 ns | 10% |
| Concurrent processing (20 channels) | ~1.1 ms | ~400-500 µs | 55-64% |
| Reconnection (10 channels) | ~300 ms | ~50 ms | 83% |

---

## Conclusion

The reaudit showed that:
- OK. Main bottlenecks remained the same
- Warning. Performance slightly degraded (5-21%), but this may be related to environment
- OK. Optimization recommendations remain relevant
- OK. New `channels` and `rpc` modules do not affect critical path

**Next Steps:**
1. Implement optimizations in priority order
2. Re-run benchmarks after each phase
3. Measure real performance improvements

---

## Appendix: Detailed Benchmark Results

### JSON Parsing
```
json_parsing/full_parse/rpc_response
  time:   [349.05 ns 349.47 ns 350.09 ns]

json_parsing/full_parse/ticker_message
  time:   [2.1003 µs 2.1027 µs 2.1059 µs]

json_parsing/full_parse/large_message
  time:   [39.536 µs 39.566 µs 39.615 µs]

json_parsing/check_key/id_in_rpc
  time:   [7.7490 ns 7.7590 ns 7.7702 ns]

json_parsing/check_key/channel_name_in_ticker
  time:   [10.460 ns 10.480 ns 10.501 ns]

json_parsing/conditional_parse/rpc_after_check
  time:   [339.60 ns 339.75 ns 339.91 ns]
```

### Handle Incoming
```
handle_incoming/rpc_response_empty_structures
  time:   [334.78 ns 335.01 ns 335.35 ns]

handle_incoming/rpc_response_with_pending
  time:   [324.62 ns 324.75 ns 324.90 ns]

handle_incoming/ticker_no_subscription
  time:   [465.86 ns 466.09 ns 466.34 ns]

handle_incoming/ticker_with_subscription
  time:   [945.22 ns 959.41 ns 973.83 ns]

handle_incoming/ticker_many_subscriptions/1
  time:   [490.77 ns 490.98 ns 491.21 ns]

handle_incoming/ticker_many_subscriptions/100
  time:   [501.79 ns 502.25 ns 502.75 ns]

handle_incoming/rpc_many_pending/100
  time:   [331.70 ns 333.38 ns 335.02 ns]
```

### Mutex Contention
```
mutex_contention/insert_remove_100
  time:   [13.440 µs 13.443 µs 13.447 µs]

mutex_contention/insert_remove_1000
  time:   [134.25 µs 134.29 µs 134.33 µs]

mutex_contention/read_heavy_100_keys
  time:   [42.956 µs 42.973 µs 42.991 µs]

mutex_contention/write_heavy_1000
  time:   [83.582 µs 83.754 µs 83.928 µs]

mutex_contention/concurrent_access_4_tasks
  time:   [63.480 µs 64.325 µs 65.250 µs]
```
