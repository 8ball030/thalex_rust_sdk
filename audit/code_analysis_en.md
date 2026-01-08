# Thalex Rust SDK Code Audit

## Project Overview

Thalex Rust SDK is a client library for working with the Thalex exchange WebSocket API. The project provides an asynchronous client for connecting to the WebSocket API, executing RPC requests, and subscribing to real-time data channels.

## Architecture and Components

### 1. Main Modules

#### `src/lib.rs`
Main library module, exports:
- `models` - data models
- `ws_client` - WebSocket client
- `channels` - subscriptions module (namespaces: market_data, accounting, conditional, etc.)
- `rpc` - RPC requests module (namespaces: market_data, trading, accounting, etc.)
- `types` - data types
- `utils` - utilities

#### `src/ws_client.rs`
Main module containing the WebSocket client implementation.

**Key Components:**

- **`WsClient`** - public client API
  - `new()` / `new_public()` / `from_env()` - client creation
  - `rpc()` - access to RPC methods (via `rpc` module)
  - `subscriptions()` - access to subscriptions (via `channels` module)
  - `send_rpc()` - JSON-RPC request execution (internal method)
  - `subscribe_channel()` - channel subscription (internal method)
  - `login()` - authentication
  - `shutdown()` - graceful shutdown

- **`connection_supervisor`** - connection supervisor
  - Automatic reconnection on connection loss
  - Re-subscription to active channels after reconnection
  - Connection error handling with exponential delay (3 seconds)

- **`resubscribe_all`** - re-subscription flow on reconnect (snapshots channels under lock, sends without holding lock)

- **`handle_incoming`** - incoming message handling
  - JSON parsing
  - RPC response routing by ID
  - Subscription routing by channel_name

#### `src/models/`
Data models generated from OpenAPI specification:

- **`Delay`** - enum for update intervals (100ms, 200ms, 500ms, 1000ms, 5000ms, 60000ms, raw)
- **`TickerData`** - ticker data structure with fields:
  - Bid/ask prices and volumes
  - Last trade price
  - Mark price and timestamp
  - IV, delta, index, forward (for options)
  - 24-hour volumes and statistics
  - Funding rate (for perpetuals)
  - Open interest, price collars
- **`TickerResponse`** - subscription response wrapper with channel_name and notification

### 2. Data Flows

#### RPC Requests (request-response)
1. Client calls methods via `client.rpc().*()` (e.g., `client.rpc().market_data().instruments()`)
2. Internally calls `send_rpc(method, params)`
3. Unique ID is generated via `AtomicU64`
4. `oneshot::channel` is created for response
5. Request is added to `pending_requests` HashMap
6. Message is sent via WebSocket
7. When response with matching ID is received, response is sent through oneshot channel
8. Timeout is determined by waiting for response through oneshot channel

#### Subscriptions (pub-sub)
1. Client calls methods via `client.subscriptions().*()` (e.g., `client.subscriptions().market_data().ticker(...)`)
2. Internally calls `subscribe_channel(scope, channel, callback)`
3. `mpsc::unbounded_channel` is created for channel
4. Subscription is added to `public_subscriptions` or `private_subscriptions` HashMap
5. Subscription command is sent to server via RPC
6. Separate task is created for callback handling
7. When message with `channel_name` is received, it's sent to corresponding channel
8. Callback is invoked in separate task

### 3. Connection Management

**Connection Supervisor:**
- Infinite reconnection loop
- On connection loss:
  - All pending RPC requests are marked as failed
  - Active subscriptions are preserved
  - After reconnection, all subscriptions are automatically restored
- Reconnection delay: 3 seconds (fixed)

**Single Connection Handling:**
- Uses `tokio::select!` for multiplexing:
  - Shutdown signals
  - Send commands
  - Incoming WebSocket messages
- Handles Ping/Pong for keepalive
- Gracefully closes connection on shutdown

## Technologies Used

- **tokio** - asynchronous runtime
- **tokio-tungstenite** - WebSocket client
- **futures-util** - utilities for working with futures
- **serde/serde_json** - JSON serialization/deserialization
- **log/simple_logger** - logging

## Design Patterns

1. **Supervisor Pattern** - supervisor manages connection lifecycle
2. **Actor Pattern** - commands are sent through channels
3. **Pub-Sub Pattern** - channel data subscriptions
4. **Request-Response Pattern** - RPC calls

## Usage Examples

See examples in `examples/` folder:
- `subscribe_ticker.rs` - ticker subscription
- `subscribe_account.rs` - account data subscription
- `simple_quoter.rs` - simple usage example
- `collect_all_trades.rs` - collect all trades
- `ohlc_streamer.rs` - OHLC data stream

Typical workflow:
1. Create client: `WsClient::new()` or `WsClient::from_env()`
2. Login: `client.login().await?`
3. RPC requests: `client.rpc().market_data().instruments().await?`
4. Subscriptions: `client.subscriptions().market_data().ticker(instrument, delay, callback).await?`
5. Wait for events: `client.run_till_event().await`
6. Graceful shutdown: `client.shutdown()`

