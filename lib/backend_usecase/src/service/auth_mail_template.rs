#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthMailContent {
    pub subject: String,
    pub body: String,
}

pub trait AuthMailTemplateProvider: Send + Sync {
    fn signup_request(&self, jwt: &str) -> AuthMailContent;
    fn reset_password_request(&self, jwt: &str) -> AuthMailContent;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultAuthMailTemplateProvider {
    base_url: String,
}

impl DefaultAuthMailTemplateProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: sanitize_base_url(base_url.into()),
        }
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Default for DefaultAuthMailTemplateProvider {
    fn default() -> Self {
        let base_url = std::env::var("FRONTEND_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());
        Self::new(base_url)
    }
}

impl AuthMailTemplateProvider for DefaultAuthMailTemplateProvider {
    fn signup_request(&self, jwt: &str) -> AuthMailContent {
        AuthMailContent {
            subject: "Verification mail".to_string(),
            body: format!(
                "Please click the link below to verify your email address.\n\n{}/verify?jwt={jwt}",
                self.base_url()
            ),
        }
    }

    fn reset_password_request(&self, jwt: &str) -> AuthMailContent {
        AuthMailContent {
            subject: "Reset Password Email".to_string(),
            body: format!(
                "Please click the link below to reset your password.\n\n{}/reset?jwt={jwt}",
                self.base_url()
            ),
        }
    }
}

fn sanitize_base_url(url: String) -> String {
    if url.ends_with('/') {
        url.trim_end_matches('/').to_string()
    } else {
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_base_url_trims_trailing_slash() {
        let sanitized = sanitize_base_url("http://example.com/".to_string());
        assert_eq!(sanitized, "http://example.com");
    }

    #[test]
    fn sanitize_base_url_keeps_base() {
        let sanitized = sanitize_base_url("http://example.com".to_string());
        assert_eq!(sanitized, "http://example.com");
    }

    #[test]
    fn default_signup_template_contains_base_url() {
        let provider = DefaultAuthMailTemplateProvider::new("http://example.com");
        let mail = provider.signup_request("token");

        assert_eq!(mail.subject, "Verification mail");
        assert!(mail.body.contains("http://example.com/verify?jwt=token"));
    }
}
