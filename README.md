# Thalex Rust SDK

This is a community-driven Rust SDK for interacting with the Thalex API, providing tools for real-time data streaming and market analysis.

## Features
- WebSocket client for real-time data streaming
- Support for various market data types (OHLC, trades, quotes)
- Easy integration with Rust applications
- Callback based event handling

Note, this SDK is not officially maintained by Thalex but is developed and supported by the community. It is a work in progress, and contributions are welcome!

## Getting Started
To get started with the Thalex Rust SDK, install the crate via Cargo:

```bash
cargo add thalex_rust_sdk
```

A client can be either public or private, depending on whether you need to access authenticated endpoints. For public access, you can create a `WsClient` without any authentication parameters.

For private access, you will need to get an API key and secret from your Thalex account and create a `WsClient` with those credentials.

A helpful instantiation method is provided to create a private client using the following environment variables:

`THALEX_PRIVATE_KEY_PATH`
`THALEX_KEY_ID`
`THALEX_ACCOUNT_ID`
`THALEX_ENVIRONMENT` (must be set to either `Mainnet` or `Testnet`)

Additionally you can create a client with a Custom environment by providing a WebSocket URL directly.

```rust
// examples/create_client.rs
use thalex_rust_sdk::{types::Environment, ws_client::WsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::Mainnet;
    // public client
    let client = WsClient::new_public(env).await.unwrap();
    // shutting down the client
    client.wait_for_connection().await;
    println!("Client connected, shutting down...");
    client.shutdown("Done!").await.unwrap();

    let private_client = WsClient::from_env().await.unwrap();
    println!("Private client created from environment variables, shutting down...");
    private_client
        .shutdown("Done with private client!")
        .await
        .unwrap();

    // custom environment
    let custom_env = Environment::Custom("wss://testnet.thalex.com/ws/api/v2".to_string());
    let custom_client = WsClient::new_public(custom_env).await.unwrap();
    println!("Custom client connected, shutting down...");
    custom_client
        .shutdown("Done with custom client!")
        .await
        .unwrap();

    Ok(())
}

```

## Subscribing to Data Streams
Once you have a `WsClient` instance, you can subscribe to various data streams. 

A callback based approach is used to handle incoming messages.

For example, to subscribe to ticker OHLC data:

```rust
// examples/subscribe_ticker.rs
use log::{Level::Info, info};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use simple_logger::init_with_level;
use thalex_rust_sdk::{
    models::Delay,
    types::{Environment, ExternalEvent},
    ws_client::WsClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    let client = WsClient::new_public(Environment::Mainnet).await.unwrap();

    let instruments = client.rpc().market_data().instruments().await.unwrap();
    info!("Total Instruments: {}", instruments.len());

    let _ = client
        .subscriptions()
        .market_data()
        .ticker("BTC-PERPETUAL", Delay::Variant1000ms, |msg| {
            // Parses into a json value initally
            async move {
            let best_bid_price: Decimal = msg.best_bid_price.unwrap();
            let best_ask_price: Decimal = msg.best_ask_price.unwrap();
            let index_price = msg.index.unwrap();

            // Check if all non-optional fields are present
            let spread = best_ask_price - best_bid_price;

            let index_delta = msg.mark_price.unwrap() - index_price;
            let index_delta_bps = if index_price != dec!(0.0) {
                (index_delta / index_price) * dec!(10000.0)
            } else {
                dec!(0.0)
            };
            info!(
                "Ticker update - Bid: {best_bid_price}, Ask: {best_ask_price} spread: {spread} index: {index_price} index_delta_bps: {index_delta_bps}"
            );
        }
    })
        .await;

    client.wait_for_connection().await;
    info!("Starting receive loop!");
    loop {
        match client.run_till_event().await {
            ExternalEvent::Connected => {
                client.resubscribe_all().await.ok();
            }
            ExternalEvent::Disconnected => continue,
            ExternalEvent::Exited => break,
        }
    }
    client.shutdown("Time to go!").await.unwrap();
    Ok(())
}

```

## Creating Orders
The SDK also provides support for authenticated endpoints, allowing you to manage your account and place orders.
```rust
// examples/create_order.rs
use thalex_rust_sdk::{
    models::{CancelParams, DirectionEnum, InsertParams, OrderTypeEnum},
    ws_client::WsClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Optionally set environment via env vars or modify WsClient::from_env for custom env
    let client = WsClient::from_env().await.unwrap();

    let order = client
        .rpc()
        .trading()
        .insert(InsertParams {
            direction: DirectionEnum::Buy,
            amount: rust_decimal_macros::dec!(0.0001),
            price: Some(rust_decimal_macros::dec!(50000.0)),
            instrument_name: Some("BTC-PERPETUAL".to_string()),
            order_type: Some(OrderTypeEnum::Limit),
            post_only: Some(true),
            reject_post_only: Some(true),
            ..Default::default()
        })
        .await
        .unwrap();

    println!("Order placed: {:?}", order);

    // cancel the order
    client
        .rpc()
        .trading()
        .cancel(CancelParams {
            order_id: Some(order.order_id.clone()),
            ..Default::default()
        })
        .await
        .unwrap();

    Ok(())
}

```



## Examples
The SDK includes several example applications demonstrating its capabilities. You can find them in the `examples`
directory. To run an example, use the following command:

```bashcargo run --example <example_name>```
Replace `<example_name>` with the name of the example file (without the `.rs` extension).

## Documentation

The client is generated automatically from the Thalex OpenAPI specification using `openapi-generator-cli`.

The api is fully documented and available [here](https://thalex.com/docs).

## Roadmap

- [x] Initial release with WebSocket support for public data streams.
- [x] Add support for private data streams and authenticated endpoints.
- [x] Implement callback based event handling.
- [x] Improve error handling and reconnection logic.
- [ ] Expand support for additional rpc endpoints.
- - [x] market data
- - [x] account management
- - [x] trading
- - [x] historical data
- - [x] market maker protection
- - [x] bot management
- - [ ] wallet operations
- - [ ] notifications
- - [ ] rfq management
- [ ] Add more examples and documentation.

## Contributing
Contributions are welcome! Please feel free to submit issues and pull requests on the GitHub repository.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Development
To build and test the SDK locally, clone the repository and run:

```bash
make test
```
This will run the test suite to ensure everything is working correctly.

For formatting and linting, use:

```bash
make fmt
make lint
```

Codegen is used to regenerate the API client from the OpenAPI specification. To do this, ensure you have `openapi-generator-cli` installed and run:

```bash
make all
```

This will update the generated code in the `src/` directory.
## Acknowledgements
This SDK is inspired by and builds upon the work of various open-source projects in the Rust ecosystem and the Thalex community.
Special thanks to the team at Thalex for their support and for providing a robust API.
