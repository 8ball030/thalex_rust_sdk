# Feedback and implementation:

The Purpose of this document is to address the findings from the recent code audit.


## Implementation of Feedback to Optimise JSON Parsing Benchmarks and WebSocket Client

Based on the audit feedback, the following changes have been implemented to enhance the clarity and performance of the JSON parsing benchmarks and WebSocket client in the Thalex Rust SDK.

[x] Added English comments alongside Russian comments in the benchmarks. This improves accessibility for a broader audience.

[x] Mutex removal for dashmaps where applicable to enhance performance and reduce contention.

[x] Resubscribe in batch for both public and private channels to minimize the number of RPC calls during reconnection.

[x] Optimized JSON parsing benchmarks to reduce overhead and improve performance. Single visitor pattern to route messages immediately without intermediate deserialization steps.

[ ] Enhanced WebSocket client to handle high-throughput scenarios more efficiently.


## Unactionable Feedback

- to_string() usage: The current implementation using `to_string()` is not needed as we are passing the &str directly to the deserializer. This is already optimal for our use case.

- Arc<str> usage: While using `Arc<str>` can be beneficial in certain scenarios, in our specific case, the overhead of converting to `Arc<str>` outweighs the benefits. The current implementation with `String` is more efficient for our use case based on benchmark results.

- bin.to_vec() usage: The current implementation using `to_vec()` is necessary for our use case as the binary data is currently a string. Using `to_vec()` is the most straightforward and efficient way to convert the string data into a byte vector for processing. Note, this route isnt actually hit in production as we only receive text messages from the WebSocket server.

