#[macro_export]
macro_rules! with_private_client {
    ($client:ident, $body:expr) => {{
        dotenv::dotenv().ok();

        let (_, _, _) = require_env!(
            "THALEX_PRIVATE_KEY_PATH",
            "THALEX_KEY_ID",
            "THALEX_ACCOUNT_ID"
        );

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let $client = WsClient::from_env().await.unwrap();

        let result = { $body };

        // check it shuts down properly
        match $client.shutdown("Test complete").await {
            Ok(_) => (),
            Err(e) => eprintln!("Error during client shutdown: {:?}", e),
        }
        result
    }};
}

#[macro_export]
macro_rules! with_public_client {
    ($client:ident, $body:expr) => {{
        let $client = WsClient::new_public().await.unwrap();

        let result = { $body };

        match $client.shutdown("Test complete").await {
            Ok(_) => (),
            Err(e) => eprintln!("Error during client shutdown: {:?}", e),
        }
        result
    }};
}

#[macro_export]
macro_rules! require_env {
    ($($var:expr),+ $(,)?) => {
        (
            $(
                match std::env::var($var) {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Skipping test: {} not set", $var);
                        return;
                    }
                }
            ),+
        )
    };
}

#[macro_export]
macro_rules! no_params_private_rpc_test {
    ($name:ident, $method:ident, $label:literal, $namespace:ident) => {
        #[tokio::test]
        #[serial_test::serial(private_rpc)]
        async fn $name() {
            let result =
                with_private_client!(client, { client.rpc().$namespace().$method().await });
            assert!(result.is_ok(), "{} failed: {:?}", $label, result.err());
        }
    };
}

#[macro_export]
macro_rules! params_private_rpc_test {
    ($name:ident, $params:expr, $method:ident, $label:literal, $namespace:ident) => {
        #[tokio::test]
        #[serial_test::serial(private_rpc)]
        async fn $name() {
            let result =
                with_private_client!(client, { client.rpc().$namespace().$method($params).await });
            assert!(result.is_ok(), "{} failed: {:?}", $label, result.err());
        }
    };
}
#[macro_export]
macro_rules! params_rpc_test {
    ($name:ident, $params:expr, $method:ident, $label:literal, $namespace:ident, $result:ident) => {
        #[tokio::test]
        #[serial_test::serial(public_rpc)]
        async fn $name() {
            let result =
                with_public_client!(client, { client.rpc().$namespace().$method($params).await });
            assert!(result.$result(), "{} failed: {:?}", $label, result.err());
        }
    };
}

#[macro_export]
macro_rules! no_params_rpc_test {
    ($name:ident, $method:ident, $label:literal, $namespace:ident, $result:ident) => {
        #[tokio::test]
        #[serial_test::serial(public_rpc)]
        async fn $name() {
            let result = with_public_client!(client, { client.rpc().$namespace().$method().await });
            assert!(result.$result(), "{} failed: {:?}", $label, result.err());
        }
    };
}
