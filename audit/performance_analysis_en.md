# Performance Analysis and Bottlenecks

**Last update:** January 2026  
**See also:** 
- [thalex_rust_sdk_performance_reaudit_2025.md](./thalex_rust_sdk_performance_reaudit_2025_en.md) - **current full reaudit report**
- [FINAL_REPORT.md](./FINAL_REPORT_en.md) - final report with recommendations

**Note:** This document describes general performance patterns. For current details and specific recommendations, see `thalex_rust_sdk_performance_reaudit_2025_en.md`. Files `*_recheck_report_*` and `*_perf_addendum_*` are historical documents.

## Identified Bottlenecks

### 1. Mutex Locks in Hot Paths

**Problem:**
- In `handle_incoming()` function, Mutex lock occurs for every incoming message
- This is a critical path, as all messages pass through this function
- At high message frequency (e.g., ticker with 100ms delay), locks can create queues
- Code uses two separate subscription tables: `public_subscriptions` and `private_subscriptions`

**Locations:**
- `handle_incoming()`: locks for accessing `pending_requests` and `public_subscriptions`/`private_subscriptions` (two separate subscription tables)
- `send_rpc()`: locks when adding/removing requests
- `subscribe_channel()`: locks when managing subscriptions

**Impact:**
- Delay in processing each message due to lock waiting
- Potential degradation with large number of concurrent RPC requests
- Contention between incoming message processing and subscription management

### 2. Excessive String Cloning

**Problem:**
- In the WebSocket reader loop, `handle_incoming(text.to_string(), ...)` performs an unnecessary `String` clone.
- In the binary branch, `String::from_utf8(bin.to_vec())` performs an unnecessary buffer copy.
- In subscribe/unsubscribe handling, `channel.to_string()` may allocate when the input is already an owned `String` (verify actual call sites).

**Impact:**
- Unnecessary memory allocations
- Data copying instead of moving
- Additional load on allocator (allocations are still expensive in Rust)

### 3. JSON Parsing for Every Message

**Problem:**
- In `handle_incoming()`, every incoming text is parsed into `serde_json::Value`
- Even if message doesn't require full parsing (e.g., only need to check for "id" or "channel_name" field)
- Benchmarks show: fast check `contains("\"id\":")` is 44 times faster than full parsing
- Note: `id` in JSON-RPC is usually a number (or `null`), so we check for marker `"id":` and then use `as_u64()`

**Impact:**
- JSON parsing is CPU-intensive operation
- At high message frequency, this can become a bottleneck
- Excessive parsing if only certain fields are needed

### 4. HashMap Operations in Critical Path

**Problem:**
- HashMap lookup under lock for every message
- With large number of subscriptions or pending requests, lookup can slow down
- Using `String` as key requires string hashing

**Impact:**
- O(1) on average, but can degrade on collisions
- String hashing is relatively expensive operation

### 5. No Batching on Re-subscription

**Problem:**
- In `resubscribe_all()`, on reconnection, separate message is sent for each channel
- Could send one command with array of all channels

**Impact:**
- More network round-trips
- More JSON serialization
- Slower subscription recovery

**Note:**  In current code, the "lock across await" issue is already fixed - snapshot of keys is taken under lock (for `public_subscriptions` and `private_subscriptions` separately), then await is performed without lock. The remaining issue is batching - sending one channel at a time instead of one request with all channels.

### 6. Fixed Reconnection Delay

**Problem:**
- In `connection_supervisor()`, fixed 3-second delay is used
- No exponential backoff
- No jitter to prevent thundering herd

**Impact:**
- Can create excessive load on network problems
- Slower recovery on temporary issues

### 7. Separate Task Creation for Each Callback

**Problem:**
- In `subscribe_channel()`, separate tokio task is created for each subscription
- With large number of subscriptions, this creates many tasks

**Impact:**
- Additional overhead for task management
- More context switches

### 8. No Buffer Pool for Reuse

**Problem:**
- Every message creates new strings and buffers
- No memory reuse

**Impact:**
- More allocations
- More pressure on allocator

### 9. Mutex Locks for instruments_cache

**Problem:**
- `WsClient` now has `instruments_cache: Arc<Mutex<HashMap<String, Instrument>>>`
- Frequent access to `round_price_to_ticks()` or `cache_instruments()` can create contention
- Lock is held during cache operations

**Impact:**
- Additional contention point with frequent use
- May block other operations during cache updates

**Locations:**
- `cache_instruments()`: lock for clearing and populating cache
- `round_price_to_ticks()`: lock for reading from cache

## Metrics for Measurement

Recommended to add metrics for:
1. Mutex lock hold time
2. Messages per second
3. Message processing latency
4. Allocation count
5. CPU usage under high load

## Optimization Priorities

### High Priority:
1. Lock optimization in `handle_incoming` (using RwLock or lock-free structures)
2. Subscription batching on reconnection
3. JSON parsing optimization (lazy parsing or streaming parser)

### Medium Priority:
4. Reducing string cloning
5. Exponential backoff for reconnection
6. Using more efficient data structures (e.g., `DashMap` for concurrent access)

### Low Priority:
7. Buffer pool
8. Callback task management optimization

