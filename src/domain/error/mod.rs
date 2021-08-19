use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserModelError {
    #[error("ユーザ名 `{0}` は3文字以上100文字未満である必要があります")]
    UserNameLengthError(String),
    #[error("ユーザID `{0}` は3文字以上100文字未満である必要があります")]
    UserIdLengthError(String),
}
