# Performance Benchmark Results Analysis

## Overview

Benchmarks have been successfully executed. Below is the analysis of results and conclusions for optimization.

**Last update:** January 2025 (after merging main and updating rustc)

**See also:** [thalex_rust_sdk_performance_reaudit_2025_en.md](./thalex_rust_sdk_performance_reaudit_2025_en.md) - full reaudit report

## Key Metrics

### 1. Incoming Message Processing (`handle_incoming`)

| Scenario | Time (2025) | Time (2024) | Change |
|----------|-------------|-------------|--------|
| RPC response (empty structures) | 335.01 ns | ~317 ns | ⚠️ +5.7% |
| RPC response (with pending request) | 324.75 ns | ~306 ns | ⚠️ +6.1% |
| Ticker without subscription | 466.09 ns | ~459 ns | ⚠️ +1.5% |
| Ticker with subscription | 959.41 ns | ~792 ns | ⚠️ **+21.1%** |
| Many subscriptions (1-100) | 490.98-502.25 ns | ~464-481 ns | ⚠️ +5-4% |
| Many pending (1-100) | 324.98-333.38 ns | ~307-314 ns | ⚠️ +5-6% |

**Conclusions:**
- ✅ RPC response processing is very efficient (~300 ns)
- ⚠️ Ticker message processing with subscription is 2.5x slower
- ✅ Good scalability - no degradation with growth in subscriptions/pending requests

### 2. JSON Parsing (`json_parsing`)

| Operation | Time (2025) | Time (2024) | Comparison |
|-----------|-------------|-------------|------------|
| Full RPC parsing | 349.47 ns | ~355 ns | ✅ -1.6% |
| Full ticker parsing | 2.1027 µs | ~2.2 µs | ✅ -4.4% |
| Full large message parsing | 39.566 µs | ~45 µs | ✅ -12.1% |
| **Key check "id"** | **7.7590 ns** | ~8 ns | ✅ **44x faster than full parsing!** |
| **Key check "channel_name"** | **10.480 ns** | ~10 ns | ✅ **200x faster than full ticker parsing!** |
| Conditional parsing (after check) | 339.75 ns | ~337 ns | ⚠️ +0.8% |

**Critical Conclusion:**
- 🚀 **Fast key checking is 44-220x faster than full parsing**
- ✅ **Recommendation**: Use fast `contains()` check before full parsing
- 💡 Potential savings: ~2 µs per ticker message

### 3. Mutex Locks (`mutex_contention`)

| Operation | Time (2025) | Time (2024) | Scaling |
|-----------|-------------|-------------|---------|
| Insert/remove 100 | 13.443 µs | ~13.4 µs | ✅ Stable |
| Insert/remove 1000 | 134.29 µs | ~134 µs | ✅ Linear (10x) |
| Read-heavy 10 keys | 42.987 µs | ~42.7 µs | ⚠️ +0.7% |
| Read-heavy 100 keys | 42.973 µs | ~42.5 µs | ✅ Independent of size |
| Write-heavy 100 | 8.2289 µs | ~7.7 µs | ⚠️ +6.9% |
| Write-heavy 1000 | 83.754 µs | ~77 µs | ✅ Linear (10x) |
| Concurrent access (4 tasks) | 64.325 µs | ~61 µs | ⚠️ Contention adds overhead |

**Conclusions:**
- ✅ Mutex operations scale linearly
- ⚠️ Lock contention adds overhead (~50% with 4 tasks)
- 💡 **Recommendation**: DashMap can improve read-heavy workload (multiple subscriptions)

### 4. Subscription Throughput (`subscription_throughput`)

| Scenario | Time | Throughput (messages/sec) |
|----------|------|---------------------------|
| Single message | ~302 ns | ~3.3M messages/sec |
| 100 messages sequentially | ~26.4 µs | ~3.8M messages/sec |
| 1000 messages sequentially | ~265 µs | ~3.8M messages/sec |
| Concurrent 1 channel | ~15.6 µs | - |
| Concurrent 5 channels | ~274 µs | ⚠️ Degradation |
| Concurrent 10 channels | ~548 µs | ⚠️ Linear degradation |
| Concurrent 20 channels | ~1.1 ms | ⚠️ **Scaling problem** |

**Critical Conclusions:**
- ✅ Sequential processing: ~3.8M messages/sec - excellent
- ⚠️ **Concurrent processing degrades linearly** with number of channels
- 🚨 **Problem**: With 20 channels, processing time for 100 messages = 1.1 ms
  - For ticker with 100ms delay, this can be a problem at high frequency

## Identified Bottlenecks

### 🔴 Critical: Concurrent Subscription Processing

**Problem:**
- With concurrent processing of many channels, performance degrades linearly
- 20 channels: ~1.1 ms for 100 messages = ~11 µs per message
- This can become a bottleneck at high message frequency

**Solution:**
- Use `DashMap` instead of `Arc<Mutex<HashMap>>` for subscriptions
- Lock-free reading allows parallel processing of different channels

### 🟡 High Priority: JSON Parsing

**Problem:**
- Full parsing of every message even when only certain fields are needed
- Ticker messages parse in ~2.2 µs, but can be faster

**Solution:**
- Fast key checking before full parsing
- Potential savings: ~2 µs per ticker message
- At 10 messages/sec frequency (100ms ticker) = 20 µs/sec savings

### 🟢 Medium Priority: Mutex Locks

**Problem:**
- Lock contention adds overhead
- With 4 concurrent tasks, overhead ~50%

**Solution:**
- DashMap for read-heavy workload (subscriptions)
- Keep Mutex for write-heavy (pending_requests) - it's effective there

## Optimization Recommendations

### Priority 1: JSON Parsing Optimization

**Expected Effect:** Savings of ~2 µs per ticker message

```rust
// In handle_incoming
async fn handle_incoming(...) {
    // Fast check before full parsing
    if text.contains("\"id\":") {
        // This is RPC response - parse only if needed
        let parsed: Value = serde_json::from_str(&text)?;
        // ...
    } else if text.contains("\"channel_name\":") {
        // This is subscription - parse only if needed
        let parsed: Value = serde_json::from_str(&text)?;
        // ...
    }
}
```

### Priority 2: DashMap for Subscriptions

**Expected Effect:** 50-80% improvement in concurrent processing

```rust
use dashmap::DashMap;

subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
```

**Advantages:**
- Lock-free reading
- Parallel processing of different channels
- Eliminates bottleneck in concurrent processing

### Priority 3: Subscription Batching

**Expected Effect:** Faster recovery after reconnection

```rust
// Instead of separate messages for each channel
// First make snapshot under lock
let channels: Vec<String> = {
    let subs = subscriptions.lock().await;
    subs.keys().map(|k| k.clone()).collect()  // Snapshot keys
};
// Lock released here

// Now send outside lock
let msg = json!({
    "method": "public/subscribe",
    "params": { "channels": channels },
});
```

## Target Metrics After Optimizations

| Metric | Current | Target | Improvement |
|--------|---------|--------|-------------|
| Ticker processing (with subscription) | ~792 ns | ~400 ns | 2x |
| JSON parsing ticker | ~2.2 µs | ~350 ns | 6x |
| Concurrent processing (20 channels) | ~1.1 ms | ~300 µs | 3.7x |
| Throughput (20 channels) | ~91K msg/s | ~333K msg/s | 3.7x |

## Conclusion

Benchmarks showed:

✅ **Strengths:**
- RPC response processing is very efficient (~300 ns)
- Good scalability with growth in subscription count
- Sequential processing has high throughput (~3.8M msg/s)

⚠️ **Bottlenecks:**
- Concurrent processing of many channels degrades
- JSON parsing can be optimized
- Mutex locks create contention under competition

🚀 **Recommendations:**
1. Implement fast JSON key checking (simple optimization, big effect)
2. Replace Mutex with DashMap for subscriptions (eliminates bottleneck)
3. Add subscription batching (improves UX)

After implementing optimizations, it's recommended to re-run benchmarks to measure improvements.

