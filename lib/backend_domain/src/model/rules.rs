use fancy_regex::Regex;

// ユーザー名のルール
pub const USER_NAME_RULE: &str = r"^[a-zA-Z0-9](?:[a-zA-Z0-9_-]{0,30}[a-zA-Z0-9])?$";

// パスワードのルール
pub const PASSWORD_RULE: &str = r"^(?=.*[a-z])(?=.*[A-Z])[a-zA-Z0-9\@\$\!\%\*\?\&]{8,64}$";

// github の id のルール
pub const GITHUB_ID_RULE: &str = r"^[a-zA-Z0-9]([a-zA-Z0-9]?|[\-]?([a-zA-Z0-9])){0,38}$";

// X の id のルール
pub const X_ID_RULE: &str = r"^[A-Za-z0-9_]{1,15}$";

// ユーザーの自己紹介のルール
pub const SELF_INTRODUCTION_RULE: &str = r".{0,10000}";

pub enum RuleType {
    UserName,
    Password,
    GitHubId,
    XId,
    SelfIntroduction,
}

impl RuleType {
    fn get_rule(&self) -> &'static str {
        match self {
            RuleType::UserName => USER_NAME_RULE,
            RuleType::Password => PASSWORD_RULE,
            RuleType::GitHubId => GITHUB_ID_RULE,
            RuleType::XId => X_ID_RULE,
            RuleType::SelfIntroduction => SELF_INTRODUCTION_RULE,
        }
    }
    pub fn validate(&self, value: &str) -> anyhow::Result<()> {
        let rule = self.get_rule();
        let re = Regex::new(rule)?;
        re.is_match(value)
            .map_err(|e| anyhow::anyhow!(e))
            .and_then(|is_match| {
                if is_match {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Invalid value: {}", value))
                }
            })
    }
}
