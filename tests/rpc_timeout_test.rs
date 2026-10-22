//! An RPC whose response never arrives fails after `RPC_TIMEOUT` instead of waiting for ever.
//! The constructor is the first RPC a client makes (it caches the instruments), so a client that
//! can never connect is the simplest way to see the bound from the outside.

use thalex_rust_sdk::types::{ClientError, Environment};
use thalex_rust_sdk::ws_client::{RPC_TIMEOUT, WsClient};

/// Nothing listens on the discard port, so no RPC ever gets a response. Time is paused, so the
/// wait costs nothing; before the bound this call never returned.
#[tokio::test(start_paused = true)]
async fn a_client_that_never_connects_fails_after_the_rpc_bound_instead_of_hanging() {
    let started = tokio::time::Instant::now();
    let result = WsClient::new(
        Environment::Custom("ws://127.0.0.1:9".to_string()),
        String::new(),
        None,
        String::new(),
    )
    .await;
    let waited = started.elapsed();

    let err = match result {
        Err(err) => err,
        Ok(_) => panic!("a client with no connection cannot have cached its instruments"),
    };
    let text = err.to_string();
    assert!(
        text.contains("public/instruments")
            && text.contains(&format!("within {} s", RPC_TIMEOUT.as_secs())),
        "the error names the RPC and the bound: {text}"
    );
    assert!(
        waited >= RPC_TIMEOUT,
        "the RPC waited the full bound before giving up: {waited:?}"
    );
}

#[test]
fn the_timeout_error_says_which_method_and_how_long() {
    let err = ClientError::Timeout {
        method: "private/insert".to_string(),
        timeout_secs: 10,
    };
    assert_eq!(
        err.to_string(),
        "RPC private/insert received no response within 10 s"
    );
}
