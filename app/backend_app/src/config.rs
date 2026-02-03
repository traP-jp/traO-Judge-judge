pub const APP_MODE_ENV: &str = "BACKEND_APP_MODE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Dev,
    Prod,
}

impl AppMode {
    pub fn from_env() -> Self {
        match std::env::var(APP_MODE_ENV).ok().as_deref() {
            Some(value)
                if value.eq_ignore_ascii_case("prod")
                    || value.eq_ignore_ascii_case("production") =>
            {
                Self::Prod
            }
            Some(value)
                if value.eq_ignore_ascii_case("dev")
                    || value.eq_ignore_ascii_case("development") =>
            {
                Self::Dev
            }
            Some(_) | None => Self::Dev,
        }
    }
}

pub fn is_prod() -> bool {
    matches!(AppMode::from_env(), AppMode::Prod)
}
