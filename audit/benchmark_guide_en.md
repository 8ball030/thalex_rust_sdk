# Performance Benchmarking Guide

## Overview

A set of benchmarks has been created to measure the performance of critical components of Thalex Rust SDK. Benchmarks use the `criterion` library, which provides statistically significant results and automatically generates reports.

## Dependency Installation

Dependencies are already added to `Cargo.toml`:
- `criterion` - benchmarking library
- `tokio-test` - utilities for testing async code

## Running Benchmarks

### Run All Benchmarks
```bash
cargo bench
```

### Run Specific Benchmark
```bash
cargo bench --bench json_parsing
cargo bench --bench handle_incoming
cargo bench --bench mutex_contention
cargo bench --bench subscription_throughput
```

### Run with Filter
```bash
cargo bench --bench json_parsing -- "full_parse"
```

## Benchmark Descriptions

### 1. `benches/json_parsing.rs`
Measures JSON parsing performance - a critical path in message processing.

**Tested Scenarios:**
- Full parsing of RPC responses (small messages)
- Full parsing of ticker messages (medium messages)
- Full parsing of large messages
- Fast key checking without full parsing
- Conditional parsing (only after key check)

**Metrics:**
- Parsing time per message
- Comparison of full parsing vs fast checking

**Using Results:**
- Determine if fast key checking should be used before full parsing
- Measure impact of message size on performance

### 2. `benches/mutex_contention.rs`
Measures Mutex lock performance under various loads.

**Tested Scenarios:**
- Write-heavy: insertion and deletion of elements
- Read-heavy: multiple reads
- Mixed load: concurrent access from multiple tasks

**Metrics:**
- Lock hold time
- Operation throughput with different element counts
- Impact of contention on performance

**Using Results:**
- Evaluate need to replace Mutex with DashMap or RwLock
- Determine optimal data structure for subscriptions and pending_requests

### 3. `benches/handle_incoming.rs`
Measures performance of `handle_incoming` function - the main message handler.

**Tested Scenarios:**
- Processing RPC responses with empty structures
- Processing RPC responses with pending requests
- Processing ticker messages without subscription
- Processing ticker messages with subscription
- Processing with many subscriptions (1, 10, 50, 100)
- Processing with many pending requests (1, 10, 50, 100)

**Metrics:**
- Single message processing time
- Impact of data structure size on performance
- Performance degradation with growth in element count

**Using Results:**
- Identify bottlenecks in message processing
- Determine optimal data structure sizes
- Evaluate impact of locks on latency

### 4. `benches/subscription_throughput.rs`
Measures subscription throughput - number of messages processed per second.

**Tested Scenarios:**
- Single message processing
- Sequential processing of 100 and 1000 messages
- Concurrent processing with different channel counts (1, 5, 10, 20)
- Real throughput measurement

**Metrics:**
- Messages per second
- Processing latency
- Impact of contention on throughput

**Using Results:**
- Determine maximum system throughput
- Evaluate if system can handle high message frequency (e.g., 100ms ticker)
- Identify bottlenecks when scaling

## Results Interpretation

### Criterion Automatically Provides:
1. **Average execution time** - main metric
2. **Standard deviation** - shows result stability
3. **Change compared to baseline** - useful when comparing optimizations
4. **HTML reports** - result visualization in `target/criterion/`

### Key Metrics to Monitor:

1. **Latency (delay)**
   - For ticker with 100ms delay, processing should be < 1ms
   - For RPC requests, processing should be < 10ms

2. **Throughput (capacity)**
   - Minimum 1000 messages/sec for single channel
   - Scaling to 10,000+ messages/sec for multiple channels

3. **Scalability**
   - Performance should not degrade linearly with growth in subscription count
   - Lock contention should not create bottlenecks

## Comparing Before and After Optimizations

### Recommended Workflow:

1. **Baseline Measurement:**
   ```bash
   cargo bench --bench handle_incoming > baseline.txt
   ```

2. **Implement Optimization**

3. **Measurement After Optimization:**
   ```bash
   cargo bench --bench handle_incoming > optimized.txt
   ```

4. **Comparison:**
   Criterion automatically compares results if benchmarks are run in the same session.

### Using Criterion for Comparison:
```bash
# Criterion automatically saves results in target/criterion/
# On re-run, compares with previous results
cargo bench --bench handle_incoming
```

## Usage Recommendations

1. **Run benchmarks on stable system**
   - Close other applications
   - Use fixed CPU frequency (if possible)
   - Run multiple times for stable results

2. **Monitor System Resources**
   - CPU usage
   - Memory usage
   - Context switches

3. **Test Realistic Scenarios**
   - Use real message sizes
   - Test with typical subscription count (10-50)
   - Consider real message frequency

4. **Document Results**
   - Save benchmark results
   - Track performance changes
   - Link optimizations with metrics

## Additional Metrics

For deeper analysis, you can add:

1. **Profiling with perf:**
   ```bash
   perf record --call-graph dwarf cargo bench --bench handle_incoming
   perf report
   ```

2. **Allocation Analysis with dhat:**
   ```toml
   [dev-dependencies]
   dhat = "0.3"
   ```

3. **Lock Monitoring:**
   Add logging of lock hold times in code.

## Interpretation Examples

### Good Result:
```
handle_incoming/ticker_with_subscription
                        time:   [1.2345 µs 1.2456 µs 1.2567 µs]
```
- Latency < 2µs - excellent for 100ms ticker
- Stable results (small standard deviation)

### Problematic Result:
```
handle_incoming/ticker_many_subscriptions/100
                        time:   [50.123 µs 55.456 µs 60.789 µs]
```
- Latency > 50µs - can be a problem at high frequency
- Large standard deviation - unstable performance
- **Action:** Consider lock optimization

### Improvement After Optimization:
```
handle_incoming/ticker_many_subscriptions/100
                        time:   [5.123 µs 5.456 µs 5.789 µs]
                        change: [-89.2% -90.1% -91.0%] (p = 0.00 < 0.05)
```
- Significant improvement (> 80%)
- Statistically significant (p < 0.05)
- **Conclusion:** Optimization successful

