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
                "traO Judgeへのご登録ありがとうございます。
以下のリンクをクリックして、メールアドレスの確認を完了してください。

🔗 認証リンク：
{}/signup/register?token={jwt}

このリンクは、60分間有効です。
期限を過ぎた場合は、お手数ですが再度登録手続きをお願いいたします。

もし本メールにお心当たりがない場合は、このメールを破棄していただいて構いません。


――――――――――――  
traO Judge 
{}
※このメールは送信専用です。返信いただいても対応できません。",
                self.base_url(),
                self.base_url()
            ),
        }
    }

    fn reset_password_request(&self, jwt: &str) -> AuthMailContent {
        AuthMailContent {
            subject: "Reset Password Email".to_string(),
            body: format!(
                "traO Judge にて、パスワード変更のリクエストを受け付けました。
以下のリンクをクリックして、パスワード変更を完了してください。

🔗 認証リンク：
{}/reset-password/form?token={jwt}

このリンクは、60分間有効です。
期限を過ぎた場合は、お手数ですが再度パスワード変更手続きをお願いいたします。

もし本メールにお心当たりがない場合は、このメールを破棄していただいて構いません。


――――――――――――
traO Judge
{}",
                self.base_url(),
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
        assert!(
            mail.body
                .contains("http://example.com/signup/register?token=token")
        );
    }
}
