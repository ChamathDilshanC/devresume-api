pub enum Role {
    Admin,
    Developer,
    Recruiter,
}

pub fn has_role(user_role: &str, required_role: Role) -> bool {
    match required_role {
        Role::Admin => user_role == "admin",
        Role::Developer => user_role == "developer" || user_role == "admin",
        Role::Recruiter => user_role == "recruiter" || user_role == "admin",
    }
}
