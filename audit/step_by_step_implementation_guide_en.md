# Step-by-Step Implementation Guide

> **Note:** The current `ws_client.rs` maintains **two** subscription maps: `public_subscriptions` and `private_subscriptions`.
> Any example below that refers to a single `subscriptions` / `self.subscriptions` map is **illustrative** and must be adapted by selecting the appropriate map (public or private).

This document contains detailed step-by-step instructions for implementing each optimization.

---

## General Recommendations

1. **Implement one optimization at a time** - don't try to implement everything at once
2. **Make commits after each optimization** - this will simplify rollback if problems occur
3. **Run tests after each step** - `cargo test`
4. **Run benchmarks** - `cargo bench` to verify improvements
5. **Check functionality** - run examples and check logs

---

## Optimization 1: Remove Unnecessary Copies (Priority 1, Cheapest Win)

### Step 1: Find Places with Unnecessary Copies

Open `src/ws_client.rs`, find function `resubscribe_all()` (re-subscription handler on reconnect).

### Step 2: Remove text.to_string()

**Find (line 592):**
```rust
Some(Ok(Message::Text(text))) => {
    handle_incoming(
        text.to_string(),
        pending_requests,
        public_subscriptions,
        private_subscriptions,
    ).await;
}
```

**Replace with:**
```rust
Some(Ok(Message::Text(text))) => {
    handle_incoming(
        text,  // text is already String
        pending_requests,
        public_subscriptions,
        private_subscriptions,
    ).await;
}
```

### Step 3: Remove bin.to_vec()

**Find (line 599):**
```rust
Some(Ok(Message::Binary(bin))) => {
    if let Ok(text) = String::from_utf8(bin.to_vec()) {
        handle_incoming(
            text,
            pending_requests,
            public_subscriptions,
            private_subscriptions,
        ).await;
    }
}
```

**Replace with:**
```rust
Some(Ok(Message::Binary(bin))) => {
    if let Ok(text) = String::from_utf8(bin) {  // Without .to_vec()
        handle_incoming(
            text,
            pending_requests,
            public_subscriptions,
            private_subscriptions,
        ).await;
    }
}
```

### Step 4: Verify

1. `cargo build` - compilation
2. `cargo test` - tests
3. Run example and verify everything works

### Expected Result
- Reduction in allocations/copies for each message
- Often visible immediately in profile

---

## Optimization 2: Batching Re-subscriptions (Priority 2, but simplest)

### Step 1: Find Re-subscription Code

Open `src/ws_client.rs`, find function `resubscribe_all()` (re-subscription handler on reconnect).

### Step 2: Replace Code

**Find:**
```rust
{
    let subs = subscriptions.lock().await;
    for channel in subs.keys() {
        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": { "channels": [channel] },
        });
        ws.send(Message::Text(msg.to_string().into())).await?;
    }
}
```

**Replace with (with snapshot, no lock during I/O):**
```rust
// First make snapshot under lock
let channels: Vec<String> = {
    let subs = subscriptions.lock().await;
    subs.keys().map(|k| k.clone()).collect()  // Snapshot keys
};
// Lock released here

// Now send outside lock
if !channels.is_empty() {
    let msg = serde_json::json!({
        "method": "public/subscribe",
        "params": { "channels": channels },
    });
    ws.send(Message::Text(msg.to_string().into())).await?;
    info!("Re-subscribed to {} channels", channels.len());
}
```

### Step 3: Verify

1. Run `cargo build` - should compile successfully
2. Run `cargo test` - all tests should pass
3. Run example `cargo run --example subscribe` - verify that re-subscription works

### Step 4: Check API

**Important:** Ensure that Thalex API supports subscription batching. Check documentation or test.

If API doesn't support:
- Roll back changes
- Or add fallback: try batch, on error send one by one

### Expected Result
- Faster restoration of subscriptions after reconnection
- Fewer log messages about re-subscription

---

## Optimization 3: JSON Parsing + Send Outside Lock (Priority 1)

### Step 1: Find handle_incoming Function

Open `src/ws_client.rs`, find function `handle_incoming` (line 334).

### Step 2: Create Backup

Copy current function to comment or separate file for reference.

### Step 3: Replace Parsing Logic

**Find function start:**
```rust
let parsed: Value = match serde_json::from_str(&text) {
    Ok(v) => v,
    Err(e) => {
        warn!("Failed to parse incoming message as JSON: {e}; raw: {text}");
        return;
    }
};
```

**Replace with (with send outside lock):**
```rust
// Fast key checking before full parsing
// IMPORTANT: In JSON-RPC, the "id" field is a number, not a string. In JSON this is "id":123, not "id":"123"
// Look for "id": (for numeric id) - without quotes after colon
if text.contains("\"id\":") {
    // This is likely an RPC response - parse only if needed
    let parsed: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse RPC response as JSON: {e}; raw: {text}");
            return;
        }
    };
    
    if let Some(id) = parsed.get("id").and_then(|v| v.as_u64()) {
        // Remove under lock, send outside lock
        let tx_opt = {
            let mut pending = pending_requests.lock().await;
            pending.remove(&id)
        };
        // Lock released here
        
        if let Some(tx) = tx_opt {
            let _ = tx.send(text);  // Send outside lock
        } else {
            warn!("Received RPC response for unknown id={id}");
        }
        return;
    }
}

if text.contains("\"channel_name\":") {
    // This is likely a subscription notification - parse only if needed
    let parsed: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse subscription message as JSON: {e}; raw: {text}");
            return;
        }
    };
    
    if let Some(channel_name) = parsed.get("channel_name").and_then(|v| v.as_str()) {
        // Clone sender under lock, send outside lock
        let sender_opt = {
            let mut subs = subscriptions.lock().await;
            subs.get_mut(channel_name).map(|s| s.clone())  // UnboundedSender clones cheaply
        };
        // Lock released here
        
        if let Some(mut sender) = sender_opt {
            if sender.send(text).is_err() {
                // If send failed, briefly take lock and remove entry
                let mut subs = subscriptions.lock().await;
                subs.remove(channel_name);
            }
        } else {
            warn!("Received message for unsubscribed channel: {channel_name}");
        }
        return;
    }
}

// If neither key found, log as unhandled
warn!("Received unhandled message (no 'id' or 'channel_name'): {text}");
```

**Important Note:** The check `contains("\"id\":")` may find `"id":` in nested objects (false positives). For full reliability, it is recommended to use Envelope parsing (see "Additional Optimizations" section).

### Step 4: Remove Old Code

Remove old processing logic after parsing (lines after parsing that are now in blocks above).

### Step 5: Verify

1. `cargo build` - compilation
2. `cargo test` - tests
3. `cargo bench --bench json_parsing` - verify improvement
4. Run example and verify message processing

### Expected Result
- Benchmarks show ~2 µs improvement for ticker messages
- Logs show correct message processing

---

## Optimization 4: DashMap for subscriptions (Priority 3, complex)

### Step 1: Add Dependency

Open `Cargo.toml`, add:
```toml
[dependencies]
dashmap = "5.5"
```

Run `cargo build` to download dependency.

### Step 2: Add use Statement

At the beginning of `src/ws_client.rs`, after other use statements:
```rust
use dashmap::DashMap;
```

### Step 3: Change Type in WsClient Structure

**Find (line 32):**
```rust
subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
```

**Replace with:**
```rust
subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
```

### Step 4: Change Initialization in connect()

**Find (line 51):**
```rust
let subscriptions = Arc::new(Mutex::new(HashMap::new()));
```

**Replace with:**
```rust
let subscriptions = Arc::new(DashMap::new());
```

### Step 5: Change subscribe()

**Find:**
```rust
// Select the appropriate subscription map: public_subscriptions or private_subscriptions
{
    let mut subs = self.public_subscriptions.lock().await; // or self.private_subscriptions
    subs.insert(channel.clone(), tx);
}
```

**Replace with:**
```rust
self.public_subscriptions.insert(channel.clone(), tx); // or self.private_subscriptions
```

### Step 6: Change unsubscribe()

**Find:**
```rust
{
    let mut subs = self.public_subscriptions.lock().await; // or self.private_subscriptions
    subs.remove(&channel);
}
```

**Replace with:**
```rust
self.public_subscriptions.remove(&channel); // or self.private_subscriptions
```

### Step 7: Change handle_incoming()

**Find (lines 359-369):**
```rust
if let Some(channel_name) = parsed.get("channel_name").and_then(|v| v.as_str()) {
    // Clone sender under lock, send outside lock
    let sender_opt = {
        let mut subs = subscriptions.lock().await;
        subs.get_mut(channel_name).map(|s| s.clone())  // UnboundedSender clones cheaply
    };
    // Lock released here
    
    if let Some(mut sender) = sender_opt {
        if sender.send(text).is_err() {
            // If send failed, briefly take lock and remove entry
            let mut subs = subscriptions.lock().await;
            subs.remove(channel_name);
        }
    } else {
        warn!("Received message for unsubscribed channel: {channel_name}");
    }
    return;
}
```

### Step 8: Change resubscribe_all() for Re-subscription

**Find (lines 257-266):**
```rust
{
    let subs = subscriptions.lock().await;
    for channel in subs.keys() {
        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": { "channels": [channel] },
        });
        ws.send(Message::Text(msg.to_string().into())).await?;
    }
}
```

**Replace with:**
```rust
// Batch all subscriptions in one message
let channels: Vec<String> = subscriptions.iter().map(|entry| entry.key().clone()).collect();
if !channels.is_empty() {
    let msg = serde_json::json!({
        "method": "public/subscribe",
        "params": { "channels": channels },
    });
    ws.send(Message::Text(msg.to_string().into())).await?;
    info!("Re-subscribed to {} channels", channels.len());
}
```

### Step 9: Verify Compilation

1. `cargo build` - should have compilation errors if something is missed
2. Fix all errors
3. `cargo test` - all tests should pass

### Step 10: Testing

1. Run example: `cargo run --example subscribe`
2. Verify that subscriptions work
3. Verify that re-subscription works after reconnection
4. Run benchmarks: `cargo bench --bench mutex_contention`
5. Run benchmarks: `cargo bench --bench subscription_throughput`

### Expected Result
- Benchmarks show 50-80% improvement in concurrent processing
- No deadlocks or race conditions
- All functional tests pass

### Potential Problems

**Problem:** Compilation error "cannot borrow as mutable"

**Solution:** DashMap `get_mut()` returns `RefMut`, need to use correctly:
```rust
if let Some(mut entry) = subscriptions.get_mut(channel_name) {
    // entry is RefMut, can be used as &mut
    if entry.send(text).is_err() {
        drop(entry); // Release RefMut before remove
        subscriptions.remove(channel_name);
    }
}
```

---

## Optimization 5: Exponential Backoff (Priority 5)

### Step 1: Decide Whether to Use fastrand

**Option A:** Use fastrand (simpler)
- Add `fastrand = "2.0"` to `Cargo.toml`

**Option B:** Use SystemTime (no dependencies)
- Doesn't require additional dependencies

Option A is recommended for simplicity.

### Step 2: Add Dependency (if chose fastrand)

In `Cargo.toml`:
```toml
[dependencies]
fastrand = "2.0"
```

### Step 3: Find connection_supervisor

Open `src/ws_client.rs`, find function `connection_supervisor` (line 182).

### Step 4: Add Backoff Variables

At the beginning of function, after `info!("Connection supervisor started for {url}");`:
```rust
let mut backoff_secs = 1u64;
const MAX_BACKOFF: u64 = 60;
```

### Step 5: Change Successful Connection Handling

**Find:**
```rust
Ok((ws_stream, _)) => {
    info!("WebSocket connected to {url}");
    // ...
}
```

**Add after `info!`:**
```rust
// Reset backoff on successful connection
backoff_secs = 1;
```

### Step 6: Replace Fixed Delay After Successful Connection

**Find (line 232):**
```rust
info!("Reconnecting to {url} after backoff");
tokio::time::sleep(std::time::Duration::from_secs(3)).await;
```

**Replace with:**
```rust
// Exponential backoff with jitter
let jitter = fastrand::u64(0..=backoff_secs);
let delay_secs = (backoff_secs + jitter).min(MAX_BACKOFF);
info!("Reconnecting to {url} after {delay_secs}s backoff");
tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF);
```

### Step 7: Replace Fixed Delay on Connection Error

**Find (line 239):**
```rust
tokio::time::sleep(std::time::Duration::from_secs(3)).await;
```

**Replace with:**
```rust
// Exponential backoff with jitter
let jitter = fastrand::u64(0..=backoff_secs);
let delay_secs = (backoff_secs + jitter).min(MAX_BACKOFF);
info!("Reconnecting to {url} after {delay_secs}s backoff");
tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF);
```

### Step 8: Verify

1. `cargo build`
2. `cargo test`
3. Run example and simulate reconnection (disconnect network)
4. Check logs - should see increasing delays

### Expected Result
- On connection errors, delay increases: 1s, 2s, 4s, 8s, ...
- On successful connection, delay resets to 1s
- Logs show different delays due to jitter

---

## Optimization 6: Reducing String Cloning (Priority 6)

### Step 1: Analyze Cloning Locations

Find all places where `to_string()` or string cloning occurs:
- `subscribe_channel()` - line 309
- `handle_incoming()` - line 643 (accepts String)
- `resubscribe_all()` - re-subscription handler on reconnect
- `connection_supervisor()` - line 503

### Step 2: Optimize subscribe()

**Find (line 309):**
```rust
let channel = channel.to_string();
```

**Check:** This cloning is necessary, as channel is needed for HashMap key.

**Optimization:** Ensure that additional cloning is not needed if channel is already String.

### Step 3: Optimize handle_incoming (optional, complex)

**Warning:** Changing signature may require changes in other places.

**Current signature:**
```rust
async fn handle_incoming(
    text: String,
    // ...
)
```

**Can change to:**
```rust
async fn handle_incoming(
    text: &str,
    // ...
)
```

**But need to:**
1. Change all calls to `handle_incoming`
2. Clone `text` only when sending: `tx.send(text.to_string())`

**Recommendation:** Leave as is if changes are too complex. Focus on other optimizations.

### Step 4: Verify

1. `cargo build`
2. `cargo test`
3. Verify that functionality hasn't changed


