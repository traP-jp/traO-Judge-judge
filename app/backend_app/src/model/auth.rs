use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignUpRequest {
    pub email: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUp {
    pub user_name: String,
    pub password: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct LogIn {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResetPassword {
    pub password: String,
    pub token: String,
}
