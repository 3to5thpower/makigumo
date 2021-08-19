use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("ユーザ `{0}` は既に存在します")]
    UserAlreadyExistsError(String),
}
