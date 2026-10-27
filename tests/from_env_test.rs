//! `WsClient::from_env` reports configuration problems as errors instead of panicking, so a
//! caller can report them and exit, or retry a transient login failure.

use serial_test::serial;
use thalex_rust_sdk::ws_client::{
    ENVIRONMENT_VAR, KEY_ID_VAR, PRIVATE_KEY_PATH_VAR, WsClient, required_env,
};

fn clear_all() {
    for name in [PRIVATE_KEY_PATH_VAR, KEY_ID_VAR, ENVIRONMENT_VAR] {
        unsafe { std::env::remove_var(name) };
    }
}

#[test]
#[serial]
fn a_required_variable_that_is_missing_or_blank_is_an_error_naming_it() {
    clear_all();
    let err = required_env(KEY_ID_VAR).unwrap_err().to_string();
    assert!(err.contains(KEY_ID_VAR), "{err}");
    unsafe { std::env::set_var(KEY_ID_VAR, "   ") };
    assert!(required_env(KEY_ID_VAR).is_err(), "blank is not a value");
    unsafe { std::env::set_var(KEY_ID_VAR, " key-1 ") };
    assert_eq!(required_env(KEY_ID_VAR).unwrap(), "key-1");
    clear_all();
}

#[tokio::test]
#[serial]
async fn a_missing_variable_fails_from_env_without_a_panic() {
    clear_all();
    let err = WsClient::from_env()
        .await
        .err()
        .expect("no variables set is an error");
    assert!(err.to_string().contains(PRIVATE_KEY_PATH_VAR), "{err}");
}

#[tokio::test]
#[serial]
async fn an_unreadable_key_file_fails_from_env_naming_the_path() {
    clear_all();
    unsafe {
        std::env::set_var(PRIVATE_KEY_PATH_VAR, "/nonexistent/thalex-key.pem");
        std::env::set_var(KEY_ID_VAR, "key-1");
        std::env::set_var(ENVIRONMENT_VAR, "Testnet");
    }
    let err = WsClient::from_env()
        .await
        .err()
        .expect("an unreadable key is an error");
    let text = err.to_string();
    assert!(text.contains("/nonexistent/thalex-key.pem"), "{text}");
    assert!(text.contains(PRIVATE_KEY_PATH_VAR), "{text}");
    clear_all();
}
