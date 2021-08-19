use crate::application::error::UserError;
use crate::domain::model::user::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::domain::service::user_service::UserService;
use anyhow::{ensure, Result};

pub struct UserRegisterService<Repository: UserRepository> {
    user_service: UserService<Repository>,
    repository: Repository,
}

impl<Repository: UserRepository> UserRegisterService<Repository> {
    pub fn register(&self, user_id: &str, user_name: &str) -> Result<()> {
        let user = User::new(user_id.to_owned(), user_name.to_owned())?;

        ensure!(
            self.user_service.exists(&user),
            UserError::UserAlreadyExistsError(user.id)
        );

        let _user = self.repository.create_user(&user)?;
        Ok(())
    }
}
