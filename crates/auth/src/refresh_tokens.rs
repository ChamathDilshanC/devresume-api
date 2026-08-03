use uuid::Uuid;

pub fn generate_refresh_token() -> String {
    format!("rt_{}", Uuid::new_v4().simple())
}
