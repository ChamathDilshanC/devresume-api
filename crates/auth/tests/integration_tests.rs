use auth::{
    create_jwt, generate_refresh_token, hash_password, verify_jwt, verify_password,
    GitHubOAuthClient, OAuthState, Permission, Role,
};
use uuid::Uuid;

#[test]
fn test_end_to_end_auth_flow() {
    // 1. Password Hashing & Registration
    let raw_password = "DeveloperSecurePassword123!";
    let password_hash = hash_password(raw_password).expect("Hashing failed");
    assert!(verify_password(raw_password, &password_hash).unwrap());

    // 2. OAuth State Generation & CSRF Protection
    let state = OAuthState::generate();
    assert!(state.verify(&state.token, 300));
    assert!(!state.verify("tampered_state", 300));

    // 3. GitHub OAuth Client Authorization URL
    let github_client = GitHubOAuthClient::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:8080/api/v1/auth/github/callback".to_string(),
    );
    let auth_url = github_client.get_authorization_url(&state.token);
    assert!(auth_url.contains("client_id=test_client_id"));
    assert!(auth_url.contains("state=st_"));

    // 4. User ID & Tokens Issuance on Callback
    let user_id = Uuid::new_v4();
    let email = "chamath@devresume.ai";
    let secret = "jwt_secret_key_32_chars_long_spec";

    let access_token = create_jwt(user_id, email, secret).expect("JWT creation failed");
    let refresh_token = generate_refresh_token();

    assert!(!access_token.is_empty());
    assert!(refresh_token.starts_with("rt_"));

    // 5. JWT Verification (simulating GET /api/v1/auth/me)
    let claims = verify_jwt(&access_token, secret).expect("JWT verification failed");
    assert_eq!(claims.sub, user_id.to_string());
    assert_eq!(claims.email, email);

    // 6. Role & Permissions Enforcement
    let dev_role = Role::Developer;
    assert!(auth::role_has_permission(dev_role, Permission::ReposSync));
    assert!(auth::role_has_permission(
        dev_role,
        Permission::ResumesGenerate
    ));
    assert!(!auth::role_has_permission(
        dev_role,
        Permission::SystemManage
    ));
}
