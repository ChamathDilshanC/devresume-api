use github::{process_github_webhook_event, verify_webhook_signature};

#[test]
fn test_webhook_push_event_processing() {
    let payload = r#"{
        "ref": "refs/heads/main",
        "before": "0000",
        "after": "1111",
        "repository": { "id": 1, "name": "DevResume-AI", "full_name": "ChamathDilshanC/DevResume-AI" },
        "commits": [{ "id": "1111", "message": "feat: webhook handler", "timestamp": "2026-08-03T00:00:00Z" }]
    }"#;

    let res = process_github_webhook_event("push", payload).expect("Webhook processing failed");
    assert!(res.contains("ChamathDilshanC/DevResume-AI"));
    assert!(res.contains("1 commits"));
}

#[test]
fn test_webhook_pull_request_event_processing() {
    let payload = r#"{
        "action": "opened",
        "number": 1,
        "pull_request": { "id": 10, "title": "feat: new feature", "state": "open" }
    }"#;

    let res = process_github_webhook_event("pull_request", payload).expect("Processing failed");
    assert!(res.contains("#1"));
    assert!(res.contains("opened"));
}

#[test]
fn test_webhook_signature_end_to_end() {
    let secret = "my_webhook_secret";
    let body = b"{\"event\":\"ping\"}";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    let sig_hex = hex::encode(mac.finalize().into_bytes());
    let header = format!("sha256={}", sig_hex);

    assert!(verify_webhook_signature(body, &header, secret));
}
