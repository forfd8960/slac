use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::{
    auth::{DecodingKey, EncodingKey},
    dto::user::{LoginReq, LoginResp, RegisterRequest, RegisterResponse, User as UserDto},
    errors::AppError,
    models::user::{CreateUser, UserRepository},
};

const MIN_NAME_LEN: usize = 6;
const MAX_NAME_LEN: usize = 200;

const MIN_PWD_LEN: usize = 8;
const MAX_PWD_LEN: usize = 20;

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

        let exist_user = self.user_store.get_by_username(&req.username).await?;
        if exist_user.is_some() {
            return Err(AppError::AlreadyExists(
                "username has registered".to_string(),
            ));
        }

        let display_name_len = req.display_name.len();
        if display_name_len == 0 || display_name_len > MAX_NAME_LEN {
            return Err(AppError::InvalidArgument(
                "display name is invalid".to_string(),
            ));
        }

        let _ = validate_password(&req.password, MIN_PWD_LEN, MAX_PWD_LEN)?;

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
            user: UserDto::from(user),
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

    pub async fn get_user(&self, user_id: i64) -> Result<UserDto, AppError> {
        let user_res = self.user_store.get_by_id(user_id).await?;
        match user_res {
            Some(user) => Ok(UserDto::from(user)),
            None => Err(AppError::NotFound("user not found".to_string())),
        }
    }
}

fn validate_password(password: &str, min_len: usize, max_len: usize) -> Result<(), AppError> {
    if password.len() < min_len || password.len() > max_len {
        return Err(AppError::InvalidArgument(
            "password too short/long".to_string(),
        ));
    }

    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch.is_ascii_lowercase() {
            has_lower = true;
        } else if ch.is_ascii_uppercase() {
            has_upper = true;
        } else if ch.is_ascii_digit() {
            has_digit = true;
        } else if ch.is_ascii_punctuation() {
            has_special = true;
        } else if !ch.is_ascii() {
            return Err(AppError::InvalidArgument(
                "password invalid(not char)".to_string(),
            ));
        }
    }

    if !has_lower {
        return Err(AppError::InvalidArgument(
            "password invalid(no lower char)".to_string(),
        ));
    }
    if !has_upper {
        return Err(AppError::InvalidArgument(
            "password invalid(no upper char)".to_string(),
        ));
    }
    if !has_digit {
        return Err(AppError::InvalidArgument(
            "password invalid(no number)".to_string(),
        ));
    }
    if !has_special {
        return Err(AppError::InvalidArgument(
            "password invalid(no special char)".to_string(),
        ));
    }

    Ok(())
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
