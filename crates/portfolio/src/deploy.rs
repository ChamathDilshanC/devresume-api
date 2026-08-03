use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentTarget {
    StaticHtmlZip,
    GitHubPages,
    CloudflarePages,
    Vercel,
    Netlify,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentResult {
    pub target: DeploymentTarget,
    pub status: String,
    pub live_url: String,
    pub deployment_id: String,
}

pub struct Deployer;

impl Deployer {
    pub async fn deploy(
        target: DeploymentTarget,
        user_id: &str,
        html_content: &str,
    ) -> Result<DeploymentResult, String> {
        if html_content.is_empty() {
            return Err("Empty HTML content provided for deployment".to_string());
        }

        let domain = match target {
            DeploymentTarget::StaticHtmlZip => "download.devresume.ai",
            DeploymentTarget::GitHubPages => "github.io",
            DeploymentTarget::CloudflarePages => "pages.dev",
            DeploymentTarget::Vercel => "vercel.app",
            DeploymentTarget::Netlify => "netlify.app",
        };

        Ok(DeploymentResult {
            target,
            status: "published".to_string(),
            live_url: format!("https://{}.{}", user_id, domain),
            deployment_id: format!("dep-{}", uuid::Uuid::new_v4()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_deployment_target() {
        let res = Deployer::deploy(DeploymentTarget::CloudflarePages, "user-7", "<html></html>")
            .await
            .unwrap();

        assert_eq!(res.status, "published");
        assert!(res.live_url.contains("pages.dev"));
    }
}
