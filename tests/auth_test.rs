// Offline checks on the login token, so a change of jsonwebtoken crypto backend cannot silently
// break login. The fixtures are throwaway keys generated for these tests only.

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use thalex_rust_sdk::auth_utils::make_auth_token;

const PKCS1_KEY: &str = include_str!("fixtures/test_rsa_pkcs1.pem");
const PKCS8_KEY: &str = include_str!("fixtures/test_rsa_pkcs8.pem");
const PUBLIC_KEY: &str = include_str!("fixtures/test_rsa_public.pem");

#[derive(Deserialize)]
struct Claims {
    iat: i64,
}

fn assert_token_verifies(private_key_pem: &str) {
    let token = make_auth_token("test-kid", &private_key_pem.to_string()).expect("signing failed");

    let header = decode_header(&token).expect("header does not decode");
    assert_eq!(header.alg, Algorithm::RS512);
    assert_eq!(header.kid.as_deref(), Some("test-kid"));

    // The token carries only iat; Thalex sets no expiry.
    let mut validation = Validation::new(Algorithm::RS512);
    validation.validate_exp = false;
    validation.required_spec_claims.clear();
    let key = DecodingKey::from_rsa_pem(PUBLIC_KEY.as_bytes()).expect("public key does not load");
    let claims = decode::<Claims>(&token, &key, &validation)
        .expect("signature does not verify")
        .claims;
    assert!(claims.iat > 0);
}

#[test]
fn signs_rs512_with_pkcs1_key() {
    assert_token_verifies(PKCS1_KEY);
}

#[test]
fn signs_rs512_with_pkcs8_key() {
    assert_token_verifies(PKCS8_KEY);
}
