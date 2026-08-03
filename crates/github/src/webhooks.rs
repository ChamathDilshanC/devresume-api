use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_webhook_signature(payload: &[u8], signature_header: &str, secret: &str) -> bool {
    if payload.is_empty() || signature_header.is_empty() || secret.is_empty() {
        return false;
    }

    let hex_signature = match signature_header.strip_prefix("sha256=") {
        Some(sig) => sig,
        None => return false,
    };

    let expected_bytes = match hex::decode(hex_signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => return false,
    };

    mac.update(payload);

    mac.verify_slice(&expected_bytes).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hmac_signature_verification() {
        let payload = b"{\"action\":\"opened\"}";
        let secret = "webhook_secret_key_123";

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload);
        let result = mac.finalize().into_bytes();
        let hex_sig = hex::encode(result);
        let header = format!("sha256={}", hex_sig);

        assert!(verify_webhook_signature(payload, &header, secret));
    }

    #[test]
    fn test_invalid_hmac_signature_rejected() {
        let payload = b"{\"action\":\"opened\"}";
        let secret = "webhook_secret_key_123";
        let invalid_header =
            "sha256=0000000000000000000000000000000000000000000000000000000000000000";

        assert!(!verify_webhook_signature(payload, invalid_header, secret));
    }

    #[test]
    fn test_missing_sha256_prefix_rejected() {
        let payload = b"hello";
        let secret = "secret";
        assert!(!verify_webhook_signature(payload, "invalid_header", secret));
    }
}
