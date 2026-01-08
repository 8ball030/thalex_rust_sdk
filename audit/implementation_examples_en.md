# Complete Code Examples for Implementing Optimizations

> **Important:** Examples in this file are illustrative. The current code uses `public_subscriptions` and `private_subscriptions`.
> If an example refers to a single `subscriptions` / `self.subscriptions` map, adapt it by selecting the appropriate map (public or private).

This document contains complete "before" and "after" code examples for each optimization.

---

## 1. Remove Unnecessary Copies of Incoming Messages

### In resubscribe_all()

**Before (lines 592, 599):**
```rust
Some(Ok(Message::Text(text))) => {
    handle_incoming(
        text.to_string(),  // Unnecessary copy
        pending_requests,
        public_subscriptions,
        private_subscriptions,
    ).await;
}
Some(Ok(Message::Binary(bin))) => {
    if let Ok(text) = String::from_utf8(bin.to_vec()) {  // Unnecessary buffer copy
        handle_incoming(
            text,
            pending_requests,
            public_subscriptions,
            private_subscriptions,
        ).await;
    }
}
```

**After:**
```rust
Some(Ok(Message::Text(text))) => {
    handle_incoming(
        text,  // text is already String
        pending_requests,
        public_subscriptions,
        private_subscriptions,
    ).await;
}
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

**Expected Effect:** Reduction in allocations/copies for each message.

---

## 2. JSON Parsing Optimization - Complete Code

### Before Optimization

```rust
async fn handle_incoming(
    text: String,
    pending_requests: &Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    let parsed: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse incoming message as JSON: {e}; raw: {text}");
            return;
        }
    };

    // RPC response: has "id"
    if let Some(id) = parsed.get("id").and_then(|v| v.as_u64()) {
        let mut pending = pending_requests.lock().await;
        if let Some(tx) = pending.remove(&id) {
            let _ = tx.send(text);
        } else {
            warn!("Received RPC response for unknown id={id}");
        }
        return;
    }

    // Subscription notification: has "channel_name"
    if let Some(channel_name) = parsed.get("channel_name").and_then(|v| v.as_str()) {
        let mut subs = subscriptions.lock().await;
        if let Some(sender) = subs.get_mut(channel_name) {
            if sender.send(text).is_err() {
                // Receiver dropped; cleanup this subscription entry.
                subs.remove(channel_name);
            }
        } else {
            warn!("Received message for unsubscribed channel: {channel_name}");
        }
        return;
    }

    warn!("Received unhandled message: {text}");
}
```

### After Optimization

```rust
async fn handle_incoming(
    text: String,
    pending_requests: &Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    // Fast key checking before full parsing
    // IMPORTANT: In JSON-RPC, the "id" field is a number, not a string. In JSON this is "id":123, not "id":"123"
    // Look for "id": (for numeric id) - without quotes after colon
    if text.contains("\"id\":") {
        // Parse only if we have "id" field
        match serde_json::from_str::<Value>(&text) {
            Ok(parsed) => {
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
            Err(e) => {
                warn!("Failed to parse RPC response as JSON: {e}; raw: {text}");
                return;
            }
        }
    }

    // Check for subscription notification
    if text.contains("\"channel_name\":") {
        // Parse only if we have "channel_name" field
        match serde_json::from_str::<Value>(&text) {
            Ok(parsed) => {
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
            Err(e) => {
                warn!("Failed to parse subscription message as JSON: {e}; raw: {text}");
                return;
            }
        }
    }

    // If neither key found, log as unhandled
    warn!("Received unhandled message (no 'id' or 'channel_name'): {text}");
}
```

**Changes:**
1. Added fast `contains()` check before parsing (look for `"id":` for numeric id)
2. Parsing is performed only if the required key is found
3. **Send outside lock** - reduces contention
4. Error handling is preserved
5. Logic for handling all cases is preserved

**Important Note:** 
- In JSON-RPC, the `id` field is a number, not a string. In JSON this is `"id":123`, not `"id":"123"`.
- The check `contains("\"id\":")` may find `"id":` in nested objects (false positives).
- For full reliability, it is recommended to use Envelope parsing (see below).

---

## 2. DashMap for subscriptions - Complete Migration

### Step 1: Add Dependency to Cargo.toml

```toml
[dependencies]
dashmap = "5.5"
# ... other dependencies
```

### Step 2: Change WsClient Structure

**Before:**
```rust
pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
}
```

**After:**
```rust
use dashmap::DashMap;

pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
}
```

### Step 3: Change Initialization in connect()

**Before:**
```rust
let subscriptions = Arc::new(Mutex::new(HashMap::new()));
```

**After:**
```rust
let subscriptions = Arc::new(DashMap::new());
```

### Step 4: Change subscribe()

**Before:**
```rust
{
    // Choose the appropriate map: public_subscriptions OR private_subscriptions
    let mut subs = self.public_subscriptions.lock().await; // or self.private_subscriptions
    subs.insert(channel.clone(), tx);
}
```

**After:**
```rust
self.public_subscriptions.insert(channel.clone(), tx); // or self.private_subscriptions
```

### Step 5: Change unsubscribe()

**Before:**
```rust
{
    // Choose the appropriate map: public_subscriptions OR private_subscriptions
    let mut subs = self.public_subscriptions.lock().await; // or self.private_subscriptions
    subs.remove(&channel);
}
```

**After:**
```rust
self.public_subscriptions.remove(&channel); // or self.private_subscriptions
```

### Step 6: Change handle_incoming()

**Before:**
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
            let mut subs = subscriptions.lock().await;
            subs.remove(channel_name);
        }
    }
}
```

**After (with DashMap, no lock needed):**
```rust
if let Some(channel_name) = parsed.get("channel_name").and_then(|v| v.as_str()) {
    if let Some(mut sender) = subscriptions.get_mut(channel_name) {
        if sender.send(text).is_err() {
            // Receiver dropped; cleanup this subscription entry.
            subscriptions.remove(channel_name);
        }
    } else {
        warn!("Received message for unsubscribed channel: {channel_name}");
    }
}
```

### Step 7: Change resubscribe_all() for Re-subscription

**Before:**
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

**After:**
```rust
// Batch all subscriptions in one message
let channels: Vec<String> = subscriptions.iter().map(|entry| entry.key().clone()).collect();
if !channels.is_empty() {
    let msg = serde_json::json!({
        "method": "public/subscribe",
        "params": { "channels": channels },
    });
    ws.send(Message::Text(msg.to_string().into())).await?;
}
```

**Important:** DashMap does not require locking for reading, but `get_mut()` returns `RefMut`, which must be used carefully.

---

## 6. Exponential Backoff - Complete Code

### Add Dependency (Optional)

If using `fastrand`:
```toml
[dependencies]
fastrand = "2.0"
```

Or use standard `rand`:
```toml
[dependencies]
rand = "0.8"
```

### Complete connection_supervisor Code with Backoff

**Before:**
```rust
async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    info!("Connection supervisor started for {url}");

    loop {
        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            break;
        }

        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                // ... connection handling ...
                
                if *shutdown_rx.borrow() {
                    break;
                }

                info!("Reconnecting to {url} after backoff");
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
            Err(e) => {
                error!("Failed to connect to {url}: {e}");
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        }
    }
}
```

**After:**
```rust
async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    info!("Connection supervisor started for {url}");

    let mut backoff_secs = 1u64;
    const MAX_BACKOFF: u64 = 60;
    const INITIAL_BACKOFF: u64 = 1;

    loop {
        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            break;
        }

        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                info!("WebSocket connected to {url}");
                
                // Reset backoff on successful connection
                backoff_secs = INITIAL_BACKOFF;

                let result = resubscribe_all(
                    &url,
                    ws_stream,
                    &mut cmd_rx,
                    &mut shutdown_rx,
                    &pending_requests,
                    &subscriptions,
                )
                .await;

                if let Err(e) = result {
                    error!("Connection error on {url}: {e}");
                }

                // Fail all pending RPCs on this connection.
                // Optimization: collect all under lock, send outside lock
                let failed_requests: Vec<_> = {
                    let mut pending = pending_requests.lock().await;
                    pending.drain().collect()  // Collect all in Vec
                };
                // Lock released here
                
                for (_, tx) in failed_requests {
                    let _ = tx.send(r#"{"error":"connection closed"}"#.to_string());  // Send outside lock
                }

                if *shutdown_rx.borrow() {
                    info!("Shutdown after connection end for {url}");
                    break;
                }

                if cmd_rx.is_closed() {
                    info!("Command channel closed for {url}, stopping supervisor");
                    break;
                }

                // Exponential backoff with jitter
                let jitter = fastrand::u64(0..=backoff_secs);
                let delay_secs = (backoff_secs + jitter).min(MAX_BACKOFF);
                info!("Reconnecting to {url} after {delay_secs}s backoff");
                tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
                backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF);
            }
            Err(e) => {
                error!("Failed to connect to {url}: {e}");
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                
                // Exponential backoff with jitter
                let jitter = fastrand::u64(0..=backoff_secs);
                let delay_secs = (backoff_secs + jitter).min(MAX_BACKOFF);
                info!("Reconnecting to {url} after {delay_secs}s backoff");
                tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
                backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF);
            }
        }
    }

    info!("Connection supervisor exited for {url}");
}
```

**Alternative without fastrand (using only standard library):**

```rust
// Instead of fastrand::u64, you can use a simple counter
use std::time::{SystemTime, UNIX_EPOCH};

// Get jitter from system time
let jitter = (SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_nanos() % (backoff_secs as u128 + 1)) as u64;
```

---

## 5. Batching Re-subscriptions - Complete Code

### Complete resubscribe_all() Code with Batching

**Before (lines 256-266):**
```rust
// Re-subscribe active channels on new connection.
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

**After (with snapshot, no lock during I/O):**
```rust
// Re-subscribe active channels on new connection (batched).
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

**If using DashMap:**
```rust
// Re-subscribe active channels on new connection (batched).
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

---

## 7. Reducing String Cloning - Specific Examples

### Example 1: subscribe_channel() - Remove Unnecessary Cloning

**Before (line 309):**
```rust
pub async fn subscribe_channel<P, F>(&self, scope: RequestScope, channel: String, callback: F) -> Result<String, Error>
where
    P: DeserializeOwned + Send + 'static,
    F: FnMut(P) + Send + 'static,
{
    let channel = channel.to_string(); // Cloning (if channel is already String)

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    {
        // Choose the appropriate map: public_subscriptions OR private_subscriptions
    let mut subs = self.public_subscriptions.lock().await; // or self.private_subscriptions
        subs.insert(channel.clone(), tx); // Cloning 2
    }

    let msg = serde_json::json!({
        "method": "public/subscribe",
        "params": {
            "channels": [channel] // Using already created string
        }
    });
    // ...
}
```

**After:**
```rust
pub async fn subscribe<F>(&self, channel: &str, mut callback: F) -> Result<(), Error>
where
    F: FnMut(String) + Send + 'static,
{
    // Clone only once when ownership is truly needed
    let channel = channel.to_string();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    {
        // Choose the appropriate map: public_subscriptions OR private_subscriptions
    let mut subs = self.public_subscriptions.lock().await; // or self.private_subscriptions
        // Use already created string, don't clone again
        subs.insert(channel.clone(), tx);
    }

    let msg = serde_json::json!({
        "method": "public/subscribe",
        "params": {
            "channels": [channel] // Use already created string
        }
    });
    // ...
}
```

**Even better - use channel directly:**
```rust
pub async fn subscribe<F>(&self, channel: &str, mut callback: F) -> Result<(), Error>
where
    F: FnMut(String) + Send + 'static,
{
    let channel_owned = channel.to_string();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    {
        // Choose the appropriate map: public_subscriptions OR private_subscriptions
    let mut subs = self.public_subscriptions.lock().await; // or self.private_subscriptions
        // Insert channel_owned, no longer needed
        subs.insert(channel_owned.clone(), tx);
    }

    let msg = serde_json::json!({
        "method": "public/subscribe",
        "params": {
            "channels": [channel_owned] // Use already created string
        }
    });
    // ...
}
```

### Example 2: handle_incoming - Accept by Reference

**Before:**
```rust
async fn handle_incoming(
    text: String, // Accepted by value
    // ...
) {
    // text is used, but doesn't own it after sending
    let _ = tx.send(text); // Transfer ownership
}
```

**After:**
```rust
async fn handle_incoming(
    text: &str, // Accepted by reference
    pending_requests: &Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    // Clone only when we need to send
    if text.contains("\"id\":") {
        match serde_json::from_str::<Value>(text) {
            Ok(parsed) => {
                if let Some(id) = parsed.get("id").and_then(|v| v.as_u64()) {
                    let mut pending = pending_requests.lock().await;
                    if let Some(tx) = pending.remove(&id) {
                        // Clone only here when we need to send
                        let _ = tx.send(text.to_string());
                    }
                    return;
                }
            }
            // ...
        }
    }
    // ...
}
```

**But need to change the call:**
```rust
// In resubscribe_all()
Some(Ok(Message::Text(text))) => {
    handle_incoming(&text, pending_requests, subscriptions).await;
}
```

---

## 6. Complete List of Dependencies for All Optimizations

```toml
[dependencies]
# Existing
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = { version = "0.28", features = ["native-tls"] }
futures-util = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4.29"
simple_logger = "5.1.0"

# New for optimizations
dashmap = "5.5"  # For concurrent HashMap
fastrand = "2.0"  # For jitter in backoff (optional, can use rand or SystemTime)
```

---

## 7. Post-Implementation Verification Checklist

### After JSON Parsing Optimization:
- [ ] Messages with "id" are processed correctly
- [ ] Messages with "channel_name" are processed correctly
- [ ] Unrecognized messages are logged
- [ ] Parsing errors are handled
- [ ] Benchmarks show improvement

### After DashMap Migration:
- [ ] Subscriptions work correctly
- [ ] Unsubscriptions work correctly
- [ ] Re-subscription on reconnection works
- [ ] No deadlocks
- [ ] Benchmarks show improvement in concurrent processing

### After Batching Re-subscriptions:
- [ ] Re-subscription works faster
- [ ] API accepts batching (check documentation)
- [ ] All channels are restored correctly

### After Exponential Backoff:
- [ ] Backoff increases on errors
- [ ] Backoff resets on successful connection
- [ ] Jitter works (different delays)
- [ ] Maximum backoff is not exceeded

---

## 8. Potential Problems and Solutions

### Problem 1: DashMap API Differs from HashMap

**Symptom:** Compilation errors when replacing.

**Solution:**
- `get_mut()` returns `RefMut`, not `&mut`
- Use `if let Some(mut entry) = map.get_mut(key)` instead of `if let Some(value) = map.get_mut(key)`
- For iteration, use `map.iter()` instead of `map.keys()`

### Problem 2: Lifetime Issues When Changing handle_incoming to &str

**Symptom:** Compilation errors about lifetime.

**Solution:**
- Ensure that `text` lives long enough
- May need to keep `String` and clone only when sending

### Problem 3: API Doesn't Support Subscription Batching

**Symptom:** Server returns error when batching.

**Solution:**
- Check API documentation
- If not supported, keep individual messages
- Or send batch, but handle error and send one by one

### Problem 4: fastrand Not Available

**Symptom:** Compilation error.

**Solution:**
- Use alternative with `SystemTime` (see example above)
- Or use `rand` crate
- Or use simple counter for deterministic jitter

---

## 11. Test Examples

### Test for JSON Parsing Optimization

```rust
#[tokio::test]
async fn test_handle_incoming_rpc_fast_path() {
    let pending = Arc::new(Mutex::new(HashMap::new()));
    let subs = Arc::new(Mutex::new(HashMap::new()));
    
    let (tx, rx) = oneshot::channel();
    pending.lock().await.insert(123, tx);
    
    let msg = r#"{"jsonrpc":"2.0","id":123,"result":{"status":"ok"}}"#;
    handle_incoming(msg.to_string(), &pending, &subs).await;
    
    let response = rx.await.unwrap();
    assert_eq!(response, msg);
}

#[tokio::test]
async fn test_handle_incoming_subscription_fast_path() {
    let pending = Arc::new(Mutex::new(HashMap::new()));
    let subs = Arc::new(Mutex::new(HashMap::new()));
    
    let (tx, mut rx) = mpsc::unbounded_channel();
    subs.lock().await.insert("test.channel".to_string(), tx);
    
    let msg = r#"{"channel_name":"test.channel","notification":{"data":"test"}}"#;
    handle_incoming(msg.to_string(), &pending, &subs).await;
    
    let received = rx.recv().await.unwrap();
    assert_eq!(received, msg);
}
```

### Test for DashMap

```rust
#[tokio::test]
async fn test_dashmap_concurrent_access() {
    use dashmap::DashMap;
    use std::sync::Arc;
    
    let map: Arc<DashMap<String, u64>> = Arc::new(DashMap::new());
    
    // Concurrent writes
    let mut handles = Vec::new();
    for i in 0..10 {
        let map_clone = map.clone();
        let handle = tokio::spawn(async move {
            map_clone.insert(format!("key_{}", i), i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    assert_eq!(map.len(), 10);
}
```

---

## Conclusion

These examples should help the customer implement the optimizations. For complex optimizations, it is recommended:

1. Implement one optimization at a time
2. Test after each one
3. Run benchmarks to verify improvements
4. Roll back changes if something doesn't work

