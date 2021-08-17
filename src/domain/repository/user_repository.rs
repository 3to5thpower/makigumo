use crate::domain::model::user::User;

pub trait UserRepository {
    fn get_user(&self) -> User;
    fn create_user(&self);
    fn update_user(&self);
    fn delete_user(&self);
}
