use crate::domain::model::user::User;
use crate::domain::repository::user_repository::UserRepository;

pub struct UserService<Repository: UserRepository> {
    repository: Repository,
}

impl<Repository: UserRepository> UserService<Repository> {
    pub fn exists(&self, user: User) -> bool {
        match self.repository.get_user(&user.id) {
            Ok(_) => true,
            Err(e) => {
                let error_kind = e.root_cause().downcast_ref::<diesel::result::Error>();
                if let Some(diesel::result::Error::NotFound) = error_kind {
                    false
                } else {
                    panic!("Error: {}", e);
                }
            }
        }
    }
}
