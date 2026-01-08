# Feedback and implementation:

The Purpose of this document is to address the findings from the recent code audit.


## Implementation of Feedback to Optimise JSON Parsing Benchmarks and WebSocket Client

Based on the audit feedback, the following changes have been implemented to enhance the clarity and performance of the JSON parsing benchmarks and WebSocket client in the Thalex Rust SDK.

[x] Added English comments alongside Russian comments in the benchmarks. This improves accessibility for a broader audience.

[x] Mutex removal for dashmaps where applicable to enhance performance and reduce contention.

[x] Resubscribe in batch for both public and private channels to minimize the number of RPC calls during reconnection.

[ ] Optimized JSON parsing benchmarks to reduce overhead and improve performance.

[ ] Enhanced WebSocket client to handle high-throughput scenarios more efficiently.