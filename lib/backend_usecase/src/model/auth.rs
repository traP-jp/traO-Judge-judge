use domain::model::rules::RuleType;

pub struct SignUpData {
    pub user_name: String,
    pub password: Option<String>,
    pub token: String,
}

impl SignUpData {
    pub fn validate(&self) -> anyhow::Result<()> {
        let mut rules = vec![(&self.user_name, RuleType::UserName)];

        if let Some(password) = &self.password {
            rules.push((password, RuleType::Password));
        }

        for (field, rule) in rules {
            rule.validate(field)?;
        }
        Ok(())
    }
}

pub struct LoginData {
    pub email: String,
    pub password: String,
}

impl LoginData {
    pub fn validate(&self) -> anyhow::Result<()> {
        let rules = vec![(&self.password, RuleType::Password)];
        for (field, rule) in rules {
            rule.validate(field)?;
        }
        Ok(())
    }
}

pub struct ResetPasswordData {
    pub password: String,
    pub token: String,
}

impl ResetPasswordData {
    pub fn validate(&self) -> anyhow::Result<()> {
        let rules = vec![(&self.password, RuleType::Password)];
        for (field, rule) in rules {
            rule.validate(field)?;
        }
        Ok(())
    }
}
