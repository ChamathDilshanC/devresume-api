pub fn check_permission(permission: &str, granted_permissions: &[&str]) -> bool {
    granted_permissions.contains(&permission)
}
