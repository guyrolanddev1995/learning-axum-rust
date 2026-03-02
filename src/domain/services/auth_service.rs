use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::entities::{User, UserRole};
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clams {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUserInfo {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub user: AuthUserInfo
}

#[derive(Clone)]
pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    jwt_secret: String,
    jwt_expiration_minutes: u64
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        jwt_secret: String,
        jwt_expiration_minutes: u64
    ) -> Self {
        Self { user_repository, jwt_secret, jwt_expiration_minutes }
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        full_name: String,
    ) -> Result<AuthResponse, DomainError> {
        if let Some(_) = self.user_repository.find_by_email(&email).await? {
            return Err(DomainError::EmailAlreadyExists);
        }

        let password_hash = self.hash_password(&password)?;

        let user = User {
            id: Uuid::new_v4(),
            email,
            password_hash,
            full_name,
            role: UserRole::Customer,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now()
        };

        let saved_user = self.user_repository.save(user).await?;
        self.build_auth_response(&saved_user)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse, DomainError> {
        let user = self.user_repository.find_by_email(email).await?
            .ok_or(DomainError::InvalidCredentialsError)?;

        if !user.is_active {
            return Err(DomainError::InvalidCredentialsError);
        }

        self.verify_password(password, &user.password_hash)?;
        self.build_auth_response(&user)
    }

    pub fn validate_token(&self, token: &str) -> Result<Clams, DomainError> {
        let validation = Validation::default();
        let token_data = decode::<Clams>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation
        )
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => DomainError::TokenExpiredError,
                _ => DomainError::InvalidTokenError
            })?;

        Ok(token_data.claims)
    }

    fn hash_password(&self, password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2.hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| DomainError::InternalError(format!("Password hashing error: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<(), DomainError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| DomainError::InternalError(format!("Password hash parsing error: {}", e)))?;

        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| DomainError::InvalidCredentialsError)
    }

    fn generate_token(&self, user: &User) -> Result<String, DomainError> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.jwt_expiration_minutes as i64);

        let claims = Clams {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: format!("{:?}", user.role),
            exp: expiration.timestamp() as usize,
            iat: now.timestamp() as usize
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        )
            .map_err(|e| DomainError::InternalError(format!("JWT encoding error: {}", e)))
    }

    fn build_auth_response(&self, user: &User) -> Result<AuthResponse, DomainError> {
        let token = self.generate_token(user)?;

        Ok(AuthResponse {
            token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_expiration_minutes as usize * 60,
            user: AuthUserInfo {
                id: user.id,
                email: user.email.clone(),
                full_name: user.full_name.clone(),
                role: format!("{:?}", user.role)
            }
        })
    }
}