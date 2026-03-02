use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::AuthResponse;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 6 characters long"))]
    pub password: String,

    #[validate(length(min = 3, message = "Full name must be at least 3 characters long"))]
    pub full_name: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 6 characters long"))]
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct UserInfoDto {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String
}

#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub token: String,
    pub token_type: String,
    pub user: UserInfoDto,
    pub expires_in: u64,
}

impl From<AuthResponse> for AuthResponseDto {
    fn from(auth: AuthResponse) -> Self {
        AuthResponseDto {
            token: auth.token,
            token_type: auth.token_type,
            user: UserInfoDto {
                id: auth.user.id.to_string(),
                email: auth.user.email,
                full_name: auth.user.full_name,
                role: auth.user.role
            },
            expires_in: auth.expires_in as u64
        }
    }
}