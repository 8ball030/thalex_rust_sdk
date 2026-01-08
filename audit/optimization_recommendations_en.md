# Optimization Recommendations

## 1. Remove Unnecessary Copies of Incoming Messages (Cheapest Win)

### Problem
In the WebSocket reader loop (handling `Message::Text` / `Message::Binary` before calling `handle_incoming()`), unnecessary allocations occur:
- `handle_incoming(text.to_string(), ...)` — unnecessary `String` clone
- `String::from_utf8(bin.to_vec())` — unnecessary buffer copy

### Solution
```rust
// Remove .to_string() - text is already String
Some(Ok(Message::Text(text))) => {
    handle_incoming(text, pending_requests, public_subscriptions, private_subscriptions).await;  // text is already String
}

// Remove .to_vec() - bin is already Vec<u8>
Some(Ok(Message::Binary(bin))) => {
    if let Ok(text) = String::from_utf8(bin) {  // without .to_vec()
        handle_incoming(text, pending_requests, public_subscriptions, private_subscriptions).await;
    }
}
```

**Expected Effect:** Reduction in allocations/copies for each message.

---

## 2. Lock Optimization

### Problem
Mutex locks in `handle_incoming` create a bottleneck at high message frequency. Additionally, the lock is held during `send()`, which increases contention.

### Solution A: Send Outside Lock (Quick Optimization)

**For subscription maps (`public_subscriptions` / `private_subscriptions`):**
```rust
// Current: lock held during send
// Code has two maps: public_subscriptions and private_subscriptions
for route in [&private_subscriptions, &public_subscriptions] {
    let mut subs = route.lock().await;
    if let Some(sender) = subs.get_mut(channel_name) {
        if sender.send(text).is_err() {  // ❌ send under lock
            subs.remove(channel_name);
        }
        return;
    }
}

// After optimization: clone sender under lock, release lock, send outside lock
let sender_opt = {
    for route in [&private_subscriptions, &public_subscriptions] {
        let mut subs = route.lock().await;
        if let Some(sender) = subs.get_mut(channel_name) {
            return Some(sender.clone());  // UnboundedSender clones cheaply
        }
    }
    None
};
// Lock released here

if let Some(mut sender) = sender_opt {
    if sender.send(text).is_err() {
        // If send failed, briefly take lock and remove entry
        for route in [&private_subscriptions, &public_subscriptions] {
            let mut subs = route.lock().await;
            if subs.remove(channel_name).is_some() {
                break;
            }
        }
    }
}
```

**For `pending_requests`:**
```rust
// Current: lock held during send
let mut pending = pending_requests.lock().await;
if let Some(tx) = pending.remove(&id) {
    let _ = tx.send(text);  // -> send under lock
}

// After optimization: remove under lock, send outside lock
let tx_opt = {
    let mut pending = pending_requests.lock().await;
    pending.remove(&id)
};
// Lock released here

if let Some(tx) = tx_opt {
    let _ = tx.send(text);  // -> Send outside lock
}
```

**For `pending_requests.drain()` on connection loss:**
```rust
// Current: drain and send under lock
let mut pending = pending_requests.lock().await;
for (_, tx) in pending.drain() {
    let _ = tx.send(r#"{"error":"connection closed"}"#.to_string());  // -> send under lock
}

// After optimization: drain under lock, send outside lock
let failed_requests: Vec<_> = {
    let mut pending = pending_requests.lock().await;
    pending.drain().collect()  // Collect all in Vec
};
// Lock released here

for (_, tx) in failed_requests {
    let _ = tx.send(r#"{"error":"connection closed"}"#.to_string());  // -> Send outside lock
}
```

**Advantages:**
- Reduces contention between incoming message processing and `subscribe/unsubscribe/call_rpc`
- Reduces lock hold time
- Safe (UnboundedSender clones cheaply)

### Solution B: Using DashMap for Concurrent Access

Replace `Arc<Mutex<HashMap>>` with `Arc<DashMap>` for lock-free reading:

```rust
use dashmap::DashMap;

pub struct WsClient {
    // ...
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,  // Two maps
    private_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,  // Two maps
    instruments_cache: Arc<DashMap<String, Instrument>>,  // Can also be optimized
    // ...
}
```

**Advantages:**
- Lock-free reading
- Parallel access to different keys
- Less contention

**Disadvantages:**
- Additional dependency
- Slightly more memory

### Solution B: Separating Read and Write Locks

Use `RwLock` instead of `Mutex`:

```rust
use tokio::sync::RwLock;

pending_requests: Arc<RwLock<HashMap<u64, ResponseSender>>>,
```

**Advantages:**
- Multiple readers simultaneously
- Less contention for read-heavy workload

**Disadvantages:**
- Writer can block all readers
- RwLock can be slower than Mutex under high contention

### Recommendation
First apply "send outside lock" (Solution A) - this is a quick and safe optimization. Then consider DashMap for `subscriptions` (read-heavy) and keep Mutex for `pending_requests` (write-heavy, but short-lived).

## 3. JSON Parsing Optimization

### Problem
Full parsing of every message even when only certain fields are needed.

### Solution A: Fast Check + Full Parsing (Simple Option)

**Important:** In JSON-RPC, the `id` field is a number, not a string. In JSON this will be `"id":123`, not `"id":"123"`.

```rust
async fn handle_incoming(
    text: String,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    private_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
) {
    // Fast check without full parsing
    // Look for marker "id": (id in JSON-RPC is usually a number or null)
    if text.contains("\"id\":") {
        // Parse only if id field exists
        if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
            if let Some(id) = parsed.get("id").and_then(|v| v.as_u64()) {
                // RPC response handling
                if let Some((_, tx)) = pending_requests.remove(&id) {
                    let _ = tx.send(text);
                }
                return;
            }
        }
    }
    
    // Similarly for channel_name
    if text.contains("\"channel_name\":") {
        // ...
    }
}
```

**Problem:** `contains()` may find `"id":` in nested objects. Two-stage check needed.

### Solution B: Envelope Parsing (Recommended)

Use a lightweight struct to parse only needed fields instead of full `Value`:

```rust
#[derive(Deserialize)]
struct Envelope<'a> {
    id: Option<u64>,
    #[serde(borrow)]
    channel_name: Option<std::borrow::Cow<'a, str>>,
}

async fn handle_incoming(
    text: &str,  // Accept by reference
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    private_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
) {
    // Fast check to filter out uninteresting messages
    if !text.contains("\"id\":") && !text.contains("\"channel_name\":"") {
        return;
    }
    
    // Light parsing of only needed fields
    match serde_json::from_str::<Envelope>(text) {
        Ok(envelope) => {
            if let Some(id) = envelope.id {
                // RPC path
                if let Some((_, tx)) = pending_requests.remove(&id) {
                    let _ = tx.send(text.to_string());  // Clone only here
                }
                return;
            }
            
            if let Some(channel_name) = envelope.channel_name {
                // Subscription path
                let channel_str = channel_name.as_ref();
                // Check both maps: public_subscriptions and private_subscriptions
                for route in [&private_subscriptions, &public_subscriptions] {
                    if let Some(mut sender) = route.get_mut(channel_str) {
                        if sender.send(text.to_string()).is_err() {
                            route.remove(channel_str);
                        }
                        return;
                    }
                }
            }
        }
        Err(e) => {
            warn!("Failed to parse message envelope: {e}");
            return;
        }
    }
}
```

**Advantages of Envelope Parsing:**
- Fewer allocations (doesn't build full Value tree)
- Less CPU for parsing
- Less pressure on allocator
- Confirms that `id` is truly top-level (avoids false positives)

**Alternative:** Use streaming JSON parser or manual parsing of only needed fields.

### Recommendation
It is recommended to use Envelope parsing (Solution B) for better performance. If simplicity is needed, start with fast check + full parsing (Solution A), but be sure to fix the `"id"` check for numeric values.

## 4. Subscription Batching

**Note:** ✅ In current code, the "lock across await" issue in `resubscribe_all()` is already fixed — channel names are snapshotted under lock and the lock is released before awaiting sends. The remaining issue is batching (sending one channel at a time instead of one request with all channels).

### Problem
Separate message for each channel on reconnection.

### Solution
Send one command with all channels. **Important:** Don't hold lock during I/O.

```rust
// In `resubscribe_all()`: take a snapshot of channels under lock, then send without holding the lock
// For public_subscriptions:
let public_channels: Vec<String> = {
    let subs = self.public_subscriptions.lock().await;
    subs.keys().cloned().collect()  // Snapshot keys
};
// Lock released here

if !public_channels.is_empty() {
    let _: RpcResponse = self.send_rpc(
        "public/subscribe",
        serde_json::json!({ "channels": public_channels }),
    ).await?;
}

// Similarly for private_subscriptions
let private_channels: Vec<String> = {
    let subs = self.private_subscriptions.lock().await;
    subs.keys().cloned().collect()
};

if !private_channels.is_empty() {
    let _: RpcResponse = self.send_rpc(
        "private/subscribe",
        serde_json::json!({ "channels": private_channels }),
    ).await?;
}
```

**Advantages:**
- Fewer network round-trips
- Faster subscription recovery
- Less JSON serialization
- **Lock not held during I/O** - reduces contention

**If using DashMap:**
```rust
// DashMap doesn't require lock for reading
// For public_subscriptions:
let public_channels: Vec<String> = public_subscriptions.iter()
    .map(|entry| entry.key().clone())
    .collect();

if !public_channels.is_empty() {
    let _: RpcResponse = self.send_rpc(
        "public/subscribe",
        serde_json::json!({ "channels": public_channels }),
    ).await?;
}

// Similarly for private_subscriptions
let private_channels: Vec<String> = private_subscriptions.iter()
    .map(|entry| entry.key().clone())
    .collect();

if !private_channels.is_empty() {
    let _: RpcResponse = self.send_rpc(
        "private/subscribe",
        serde_json::json!({ "channels": private_channels }),
    ).await?;
}
```

## 5. Reducing String Cloning

### Problem
Excessive string creation in multiple places.

### Solutions

#### A. Use `&str` Where Possible
```rust
pub async fn subscribe<F>(&self, channel: &str, mut callback: F) -> Result<(), Error>
where
    F: FnMut(String) + Send + 'static,
{
    // Don't clone channel until necessary
    let channel = channel.to_string(); // Only if ownership is really needed
    // ...
}
```

#### B. Pass Strings by Reference in handle_incoming
```rust
async fn handle_incoming(
    text: &str, // Change to &str
    // ...
) {
    // Use text directly, clone only when necessary
}
```

#### C. Use `Cow<str>` for Conditional Ownership
```rust
use std::borrow::Cow;

async fn handle_incoming(
    text: Cow<'_, str>,
    // ...
)
```

## 6. Exponential Backoff for Reconnection

### Problem
Fixed 3-second delay.

### Solution
```rust
async fn connection_supervisor(
    // ...
) {
    let mut backoff_secs = 1u64;
    const MAX_BACKOFF: u64 = 60;
    
    loop {
        // ...
        if let Err(e) = result {
            error!("Connection error on {url}: {e}");
            
            // Fail all pending RPCs
            // ...
            
            if *shutdown_rx.borrow() {
                break;
            }
            
            // Exponential backoff with jitter
            let jitter = fastrand::u64(0..=backoff_secs);
            tokio::time::sleep(std::time::Duration::from_secs(backoff_secs + jitter)).await;
            backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF);
        } else {
            // Reset backoff on successful connection
            backoff_secs = 1;
        }
    }
}
```

**Advantages:**
- Less load on server during problems
- Faster recovery on temporary issues
- Jitter prevents thundering herd

## 6. Data Structure Optimization for Subscriptions

### Problem
HashMap with String keys requires hashing.

### Solution
If channel names have limited pattern set, can use more efficient structure:

```rust
// If channel names are known in advance, can use enum
#[derive(Hash, Eq, PartialEq, Clone)]
enum Channel {
    Ticker(String, Delay), // "ticker.BTC-PERPETUAL.100ms"
    // other channel types
}
```

Or use interned strings to reduce allocations:

```rust
use string_cache::DefaultAtom as Atom;

subscriptions: Arc<DashMap<Atom, mpsc::UnboundedSender<String>>>,
```

## 8. Buffer Pool (Optional)

### Problem
Many allocations for message strings.

### Solution
Use `bytes::Bytes` or buffer pool:

```rust
use bytes::Bytes;

// In handle_incoming accept Bytes instead of String
async fn handle_incoming(
    text: Bytes,
    // ...
)
```

Or use `bumpalo` for arena allocation in critical paths.

## 9. Pre-allocating HashMap Capacity

### Problem
HashMap grows dynamically, which can cause reallocations with many subscriptions or pending requests.

### Solution
```rust
// On initialization
let pending_requests = Arc::new(Mutex::new(HashMap::with_capacity(1024)));  // Expected upper bound
let subscriptions = Arc::new(Mutex::new(HashMap::with_capacity(128)));  // Typical subscription count
```

**Additionally:** When `subscribe()` is called and channel already exists, don't silently overwrite sender. Can:
- Return Err/log warning and replace carefully, or
- Remove old sender and let it finish.

**Expected Effect:** Fewer reallocations, better resource control.

---

## 10. Metrics and Profiling

### Recommendation
Add metrics for performance monitoring:

```rust
use std::time::Instant;

async fn handle_incoming(
    text: String,
    // ...
) {
    let start = Instant::now();
    
    // ... processing ...
    
    let duration = start.elapsed();
    if duration.as_millis() > 10 {
        warn!("Slow message handling: {:?}", duration);
    }
}
```

Or use `tracing` for structured logging and profiling.

## Implementation Prioritization (Minimum Risk → Maximum Profit)

1. **Immediately (Safe and Fast):**
   - Remove unnecessary copies (optimization 1) - cheapest win
   - Subscription batching with snapshot (optimization 4) - safe, reduces lock+await issue

2. **High Priority (Safe, Noticeable Effect):**
   - Send outside lock (optimization 2, solution A) - safe, usually gives noticeable effect under contention
   - JSON parsing optimization with Envelope (optimization 3, solution B) - medium risk, but big win

3. **Medium Priority:**
   - DashMap for subscriptions (optimization 2, solution B) - eliminates bottleneck, but requires testing
   - Exponential backoff (optimization 6) - more about reliability than nanoseconds
   - Pre-allocating HashMap capacity (optimization 9) - low risk

4. **Low Priority:**
   - Reducing string cloning (optimization 5) - requires care with lifetimes
   - Buffer pool (optimization 8) - only if profiling shows necessity

## Testing Optimizations

Recommended:
1. Create benchmark tests for performance measurement
2. Use `criterion` for benchmarks
3. Test under load (high message frequency)
4. Measure latency and throughput before and after optimizations

