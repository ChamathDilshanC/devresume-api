use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum PortfolioTheme {
    Minimal,
    #[default]
    Modern,
    Glass,
    Developer,
    Corporate,
}

impl PortfolioTheme {
    pub fn name(&self) -> &'static str {
        match self {
            PortfolioTheme::Minimal => "minimal-clean-white",
            PortfolioTheme::Modern => "modern-dark-cyan",
            PortfolioTheme::Glass => "glassmorphism-neon-purple",
            PortfolioTheme::Developer => "developer-monospaced-terminal",
            PortfolioTheme::Corporate => "corporate-slate-blue",
        }
    }

    pub fn css_variables(&self) -> &'static str {
        match self {
            PortfolioTheme::Minimal => {
                "--bg: #ffffff; --text: #111827; --accent: #2563eb; --card: #f9fafb;"
            }
            PortfolioTheme::Modern => {
                "--bg: #0f172a; --text: #f8fafc; --accent: #06b6d4; --card: #1e293b;"
            }
            PortfolioTheme::Glass => {
                "--bg: #180b28; --text: #ffffff; --accent: #a855f7; --card: rgba(255,255,255,0.05);"
            }
            PortfolioTheme::Developer => {
                "--bg: #0d1117; --text: #c9d1d9; --accent: #3fb950; --card: #161b22;"
            }
            PortfolioTheme::Corporate => {
                "--bg: #f8fafc; --text: #1e293b; --accent: #0f172a; --card: #ffffff;"
            }
        }
    }
}
