use crate::domain::model::user::User;
use anyhow::Result;

pub trait UserRepository {
    fn get_user(&self, id: usize) -> Result<User>;
    fn get_users(&self) -> Result<Vec<User>>;
    fn create_user(&self, user: &User) -> Result<User>;
    fn update_user(&self, user: &User) -> Result<User>;
    fn delete_user(&self, user: &User) -> Result<()>;
}
