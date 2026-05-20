//! Integration tests for [`SolClient::modify_client_info`].
//!
//! These tests exercise the issue-#8 fix: `modify_client_info` must not
//! return until the broker has confirmed the property change (i.e. the
//! `SOLCLIENT_SESSION_EVENT_MODIFYPROP_OK` event has fired and the P2P
//! topic migration is complete). Before the fix, calling
//! `send_request_async` immediately after `modify_client_info` could
//! intermittently time out for up to ~3 s.
//!
//! All tests in this file talk to a real Solace broker and are gated on
//! the same env vars used elsewhere in the project:
//!
//! ```text
//! SOLACE_HOST     e.g. "tcp://localhost:55555"
//! SOLACE_VPN
//! SOLACE_USERNAME
//! SOLACE_PASSWORD
//! ```
//!
//! They also require a peer service responding on
//! `SOLACE_MODIFY_REPLY_TOPIC` (default `api/v1/ping`) so the
//! send_request_async path has something to reply. Without a replier the
//! "no race" assertion can't be made and the test would be meaningless,
//! so the tests are `#[ignore]` by default. Run them manually with:
//!
//! ```bash
//! cargo test -p rsolace --test modify_client_info -- --ignored --nocapture
//! ```

use std::time::Duration;

use rsolace::solclient::SolClient;
use rsolace::solmsg::SolMsg;
use rsolace::types::{SolClientLogLevel, SolClientReturnCode};
use rsolace::SessionProps;

fn env_or_skip(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(v) if !v.is_empty() => Some(v),
        _ => None,
    }
}

fn live_session_props(initial_client_name: &str) -> Option<SessionProps> {
    let host = env_or_skip("SOLACE_HOST")?;
    let vpn = env_or_skip("SOLACE_VPN")?;
    let username = env_or_skip("SOLACE_USERNAME")?;
    let password = env_or_skip("SOLACE_PASSWORD")?;
    Some(
        SessionProps::default()
            .host(&host)
            .vpn(&vpn)
            .username(&username)
            .password(&password)
            .client_name(initial_client_name)
            .reapply_subscriptions(true)
            .connect_retries(1)
            .connect_timeout_ms(5000)
            .compression_level(0),
    )
}

/// Smoke test: after `modify_client_info` returns Ok, a subsequent
/// `send_request_async` to a known replier must complete within 5s.
/// Loops 10 times so any non-deterministic race window has a fair
/// chance to manifest.
#[test]
#[ignore = "requires live Solace broker + replier; run with --ignored"]
fn modify_client_info_then_request_no_race() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .try_init();

    let _ = dotenvy::dotenv();

    let reply_topic =
        std::env::var("SOLACE_MODIFY_REPLY_TOPIC").unwrap_or_else(|_| "api/v1/ping".to_string());

    let props = match live_session_props("rsolace-modify-test") {
        Some(p) => p,
        None => {
            eprintln!(
                "skipping: SOLACE_HOST/VPN/USERNAME/PASSWORD not set; run with --ignored after exporting them"
            );
            return;
        }
    };

    let mut client = SolClient::new(SolClientLogLevel::Notice).expect("SolClient::new");
    let connected = client.connect(props);
    assert!(connected, "failed to connect to broker");

    // Give the API a beat to finish session-up bookkeeping.
    std::thread::sleep(Duration::from_millis(200));

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");

    for i in 0..10 {
        let new_name = format!("rsolace-modify-test-{}", i);
        let rc = client.modify_client_info(None, Some(&new_name));
        assert_eq!(
            rc,
            SolClientReturnCode::Ok,
            "iteration {}: modify_client_info({}) returned {:?}",
            i,
            new_name,
            rc
        );

        let mut msg = SolMsg::new().expect("SolMsg::new");
        msg.set_topic(&reply_topic);
        msg.set_delivery_to_one(true);

        // 5s upper bound for the round-trip. With the fix, this should
        // succeed on every iteration. Without the fix, the first
        // ~0-3s after modify_client_info can drop replies because the
        // P2P subscription for the new client_name hasn't migrated yet.
        let reply = runtime.block_on(async {
            tokio::time::timeout(Duration::from_secs(5), client.send_request_async(&msg)).await
        });

        match reply {
            Ok(Ok(_msg)) => {
                tracing::info!(iteration = i, "request/reply ok");
            }
            Ok(Err(e)) => panic!("iteration {}: send_request_async error: {:?}", i, e),
            Err(_elapsed) => panic!(
                "iteration {}: send_request_async timed out (5s); modify_client_info race not fixed",
                i
            ),
        }
    }

    client.disconnect();
}

/// Negative path: a `modify_client_info` call with no fields supplied
/// must fail fast (NotFound) without registering or leaking a waiter.
/// This test does not require a broker.
#[test]
fn modify_client_info_no_props_returns_not_found() {
    let mut client = SolClient::new(SolClientLogLevel::Notice).expect("SolClient::new");
    let rc = client.modify_client_info(None, None);
    assert_eq!(rc, SolClientReturnCode::NotFound);
}
