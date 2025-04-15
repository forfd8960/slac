use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{DecodingKey, EncodingKey},
    dto::user::{LoginReq, LoginResp, RegisterRequest, RegisterResponse, User as UserDto},
    errors::AppError,
    models::user::{CreateUser, User, UserRepository},
};

const MIN_NAME_LEN: usize = 6;
const MAX_NAME_LEN: usize = 200;

lazy_static! {
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*()\-_=+{};:,<.>]).{8,}$")
            .unwrap();
}

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub username: String,
    pub avatar_url: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResp {
    pub user: User,
}

pub struct UserService<'a> {
    user_store: &'a UserRepository<'a>,
    ek: &'a EncodingKey,
    dk: &'a DecodingKey,
}

impl<'a> UserService<'a> {
    pub fn new(user_store: &'a UserRepository, ek: &'a EncodingKey, dk: &'a DecodingKey) -> Self {
        Self { user_store, ek, dk }
    }

    pub async fn create_user(&self, req: &RegisterRequest) -> Result<RegisterResponse, AppError> {
        let name_len = req.username.len();
        if name_len == 0 || name_len < MIN_NAME_LEN || name_len > MAX_NAME_LEN {
            return Err(AppError::InvalidArgument(
                "user name is invalid".to_string(),
            ));
        }

        let display_name_len = req.display_name.len();
        if display_name_len == 0 || display_name_len > MAX_NAME_LEN {
            return Err(AppError::InvalidArgument(
                "display name is invalid".to_string(),
            ));
        }

        if !validate_password(&req.password) {
            return Err(AppError::InvalidArgument("password is invalid".to_string()));
        }

        let pwd_hash = hash_password(&req.password)?;
        let user = self
            .user_store
            .create(&CreateUser {
                username: req.username.clone(),
                avatar_url: req.avatar.clone(),
                password_hash: pwd_hash,
                display_name: req.display_name.clone(),
                is_active: true,
            })
            .await?;

        Ok(RegisterResponse {
            user: UserDto {
                id: user.id,
                username: user.username,
                avatar_url: user.avatar_url,
                display_name: user.display_name,
                is_active: user.is_active,
                created_at: user.created_at,
                updated_at: user.updated_at,
            },
        })
    }

    pub async fn login(&self, req: &LoginReq) -> Result<LoginResp, AppError> {
        let user_res = self.user_store.get_by_username(&req.username).await?;
        match user_res {
            Some(user) => {
                let verified = verify_password(&req.password, &user.password_hash)?;
                if !verified {
                    return Err(AppError::Unauthorized("password is correct".to_string()));
                }

                let tk = self.ek.sign(UserDto::from(user))?;
                Ok(LoginResp { token: tk })
            }
            None => Err(AppError::NotFound("user not found".to_string())),
        }
    }
}

fn validate_password(pwd: &str) -> bool {
    PASSWORD_REGEX.is_match(pwd)
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
