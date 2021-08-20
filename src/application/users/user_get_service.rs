use crate::domain::model::user::User;
use crate::domain::repository::user_repository::UserRepository;
use anyhow::Result;

pub struct GetUserService<Repository: UserRepository> {
    repository: Repository,
}

impl<Repository: UserRepository> GetUserService<Repository> {
    pub fn get_user(&self, user_id: &str) -> Result<User> {
        self.repository.get_user(user_id)
    }

    pub fn get_users(&self) -> Result<Vec<User>> {
        self.repository.get_users()
    }
}
