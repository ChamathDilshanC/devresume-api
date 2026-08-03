use crate::rbac::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    UsersRead,
    UsersWrite,
    ReposSync,
    ResumesGenerate,
    SystemManage,
    PortfolioPublish,
    ProfileUpdate,
    CandidatesView,
    ResumesView,
    AnalyticsView,
    SearchUse,
}

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::UsersRead => "users:read",
            Permission::UsersWrite => "users:write",
            Permission::ReposSync => "repos:sync",
            Permission::ResumesGenerate => "resumes:generate",
            Permission::SystemManage => "system:manage",
            Permission::PortfolioPublish => "portfolio:publish",
            Permission::ProfileUpdate => "profile:update",
            Permission::CandidatesView => "candidates:view",
            Permission::ResumesView => "resumes:view",
            Permission::AnalyticsView => "analytics:view",
            Permission::SearchUse => "search:use",
        }
    }
}

pub fn get_permissions_for_role(role: Role) -> Vec<Permission> {
    match role {
        Role::Admin => vec![
            Permission::UsersRead,
            Permission::UsersWrite,
            Permission::ReposSync,
            Permission::ResumesGenerate,
            Permission::SystemManage,
            Permission::PortfolioPublish,
            Permission::ProfileUpdate,
            Permission::CandidatesView,
            Permission::ResumesView,
            Permission::AnalyticsView,
            Permission::SearchUse,
        ],
        Role::Developer => vec![
            Permission::ReposSync,
            Permission::ResumesGenerate,
            Permission::PortfolioPublish,
            Permission::ProfileUpdate,
        ],
        Role::Recruiter => vec![
            Permission::CandidatesView,
            Permission::ResumesView,
            Permission::AnalyticsView,
            Permission::SearchUse,
        ],
    }
}

pub fn check_permission(permission: &str, granted_permissions: &[&str]) -> bool {
    granted_permissions.contains(&permission)
}

pub fn role_has_permission(role: Role, permission: Permission) -> bool {
    get_permissions_for_role(role).contains(&permission)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_has_all_permissions() {
        assert!(role_has_permission(Role::Admin, Permission::SystemManage));
        assert!(role_has_permission(Role::Admin, Permission::UsersRead));
        assert!(role_has_permission(Role::Admin, Permission::SearchUse));
    }

    #[test]
    fn test_developer_permissions() {
        assert!(role_has_permission(Role::Developer, Permission::ReposSync));
        assert!(role_has_permission(
            Role::Developer,
            Permission::ResumesGenerate
        ));
        assert!(role_has_permission(
            Role::Developer,
            Permission::PortfolioPublish
        ));
        assert!(role_has_permission(
            Role::Developer,
            Permission::ProfileUpdate
        ));
        assert!(!role_has_permission(
            Role::Developer,
            Permission::SystemManage
        ));
    }

    #[test]
    fn test_recruiter_permissions() {
        assert!(role_has_permission(
            Role::Recruiter,
            Permission::CandidatesView
        ));
        assert!(role_has_permission(
            Role::Recruiter,
            Permission::ResumesView
        ));
        assert!(role_has_permission(
            Role::Recruiter,
            Permission::AnalyticsView
        ));
        assert!(role_has_permission(Role::Recruiter, Permission::SearchUse));
        assert!(!role_has_permission(Role::Recruiter, Permission::ReposSync));
    }
}
