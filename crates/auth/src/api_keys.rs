use uuid::Uuid;

pub fn generate_api_key() -> String {
    format!("devresume_sk_{}", Uuid::new_v4().simple())
}
