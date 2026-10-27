//! [`SolClient::send_multiple_msg_written`] guard paths that do not need a broker.

use rsolace::solclient::{SolClient, SEND_MULTIPLE_LIMIT};
use rsolace::solmsg::SolMsg;
use rsolace::types::{SolClientLogLevel, SolClientReturnCode};

#[test]
fn limit_matches_solclient() {
    assert_eq!(SEND_MULTIPLE_LIMIT, 50);
}

#[test]
fn empty_batch_is_ok_without_calling_the_api() {
    let client = SolClient::new(SolClientLogLevel::Warning).unwrap();
    assert_eq!(
        client.send_multiple_msg_written(&[]),
        (SolClientReturnCode::Ok, 0)
    );
}

#[test]
fn over_limit_is_refused_instead_of_panicking() {
    let client = SolClient::new(SolClientLogLevel::Warning).unwrap();
    let msgs: Vec<SolMsg> = (0..=SEND_MULTIPLE_LIMIT)
        .map(|_| SolMsg::new().unwrap())
        .collect();
    let refs: Vec<&SolMsg> = msgs.iter().collect();
    assert_eq!(
        client.send_multiple_msg_written(&refs),
        (SolClientReturnCode::Fail, 0)
    );
    assert_eq!(client.send_multiple_msg(&refs), SolClientReturnCode::Fail);
}

#[test]
fn unconnected_session_writes_nothing() {
    let client = SolClient::new(SolClientLogLevel::Warning).unwrap();
    let mut msg = SolMsg::new().unwrap();
    msg.set_topic("rsolace/test/send_multiple");
    msg.set_binary_attachment(b"x");
    let (code, written) = client.send_multiple_msg_written(&[&msg, &msg]);
    assert_ne!(code, SolClientReturnCode::Ok);
    assert_eq!(written, 0);
}
