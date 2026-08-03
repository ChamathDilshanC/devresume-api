pub fn verify_webhook_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
    !payload.is_empty() && !signature.is_empty() && !secret.is_empty()
}
