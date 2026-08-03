use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Developer,
    Recruiter,
}

impl std::str::FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "developer" | "dev" => Ok(Role::Developer),
            "recruiter" => Ok(Role::Recruiter),
            _ => Err(()),
        }
    }
}

impl Role {
    pub fn parse(role_str: &str) -> Option<Self> {
        role_str.parse().ok()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Developer => "developer",
            Role::Recruiter => "recruiter",
        }
    }
}

pub fn has_role(user_role: &str, required_role: Role) -> bool {
    let parsed_role = match Role::parse(user_role) {
        Some(r) => r,
        None => return false,
    };

    match required_role {
        Role::Admin => parsed_role == Role::Admin,
        Role::Developer => parsed_role == Role::Developer || parsed_role == Role::Admin,
        Role::Recruiter => parsed_role == Role::Recruiter || parsed_role == Role::Admin,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_parsing() {
        assert_eq!(Role::parse("admin"), Some(Role::Admin));
        assert_eq!(Role::parse("DEVELOPER"), Some(Role::Developer));
        assert_eq!(Role::parse("recruiter"), Some(Role::Recruiter));
        assert_eq!(Role::parse("invalid_role"), None);
    }

    #[test]
    fn test_admin_inherits_all_roles() {
        assert!(has_role("admin", Role::Admin));
        assert!(has_role("admin", Role::Developer));
        assert!(has_role("admin", Role::Recruiter));
    }

    #[test]
    fn test_developer_role_hierarchy() {
        assert!(has_role("developer", Role::Developer));
        assert!(!has_role("developer", Role::Admin));
        assert!(!has_role("developer", Role::Recruiter));
    }

    #[test]
    fn test_recruiter_role_hierarchy() {
        assert!(has_role("recruiter", Role::Recruiter));
        assert!(!has_role("recruiter", Role::Admin));
        assert!(!has_role("recruiter", Role::Developer));
    }
}
