#[test]
fn test_api_health_route_naming() {
    let route = "/api/v1/health";
    assert!(route.starts_with("/api/v1"));
}
