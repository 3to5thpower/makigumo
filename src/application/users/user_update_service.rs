use crate::application::error::UserError;
use crate::domain::model::user::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::domain::service::user_service::UserService;
use anyhow::{ensure, Result};

pub struct UserUpdateService<Repository: UserRepository> {
    user_service: UserService<Repository>,
    repository: Repository,
}

impl<Repository: UserRepository> UserUpdateService<Repository> {
    pub fn update_user_name(&self, user_id: &str, new_user_name: &str) -> Result<User> {
        let mut user = self.repository.get_user(user_id)?;
        user.name = new_user_name.to_owned();

        self.repository.update_user(&user)
    }

    pub fn update_user_id(&self, user_id: &str, new_user_id: &str) -> Result<User> {
        let mut user = self.repository.get_user(user_id)?;
        user.id = new_user_id.to_owned();

        ensure!(
            !self.user_service.exists(&user),
            UserError::UserAlreadyExistsError(user.id)
        );
        self.repository.update_user(&user)
    }
}
