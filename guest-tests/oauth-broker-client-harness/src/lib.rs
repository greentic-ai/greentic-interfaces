#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
wit_bindgen::generate!({
    path: "wit",
    world: "test:oauth-broker-client-harness/harness@0.1.0",
    generate_all,
});

#[cfg(target_arch = "wasm32")]
struct Harness;

#[cfg(target_arch = "wasm32")]
impl exports::test::oauth_broker_client_harness::harness_api::Guest for Harness {
    fn run() -> Vec<String> {
        let scopes = vec!["openid".to_string(), "profile".to_string()];
        let consent = test::oauth_broker_client_harness::broker_v1::get_consent_url(
            "provider", "subject", &scopes, "/cb", "{}",
        );
        let token =
            test::oauth_broker_client_harness::broker_v1::get_token("provider", "subject", &scopes);
        let exchanged = test::oauth_broker_client_harness::broker_v1::exchange_code(
            "provider",
            "subject",
            "auth-code",
            "/cb",
        );

        vec![consent, token, exchanged]
    }
}

#[cfg(target_arch = "wasm32")]
export!(Harness);
