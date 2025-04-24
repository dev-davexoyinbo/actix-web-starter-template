use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use tracing::instrument;

use crate::common_error::CommonError;

pub struct PasswordUtils {
    salt: SaltString,
}

impl PasswordUtils {
    #[instrument(skip_all)]
    pub fn initialize(salt: String) -> Result<PasswordUtils, CommonError> {
        let salt = SaltString::encode_b64(salt.as_bytes())
            .map_err(|err| CommonError::PasswordError(format!("Error creating salt: {}", err)))?;

        Ok(PasswordUtils { salt })
    }

    #[instrument(skip_all)]
    pub fn hash_password(&self, password: &str) -> Result<String, CommonError> {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &self.salt)
            .map_err(|err| CommonError::PasswordError(format!("{}", err)))?
            .to_string();

        Ok(hashed_password)
    }

    #[instrument(skip_all)]
    pub fn verify_hashed_password(
        &self,
        hashed_password: &str,
        needle: &str,
    ) -> Result<bool, CommonError> {
        let parsed_hash = PasswordHash::new(hashed_password)
            .map_err(|err| CommonError::PasswordError(format!("{}", err)))?;

        let Some(salt) = parsed_hash.salt else {
            return Err(CommonError::PasswordError(
                "Password generated in insecure environment".to_string(),
            ));
        };

        if salt.to_string() != self.salt.to_string() {
            return Err(CommonError::PasswordError(
                "Password invalidated".to_string(),
            ));
        }

        let password_correct = Argon2::default()
            .verify_password(needle.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(password_correct)
    }
}
