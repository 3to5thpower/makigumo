use crate::domain::model::user::User;
use crate::infrastructures::database::schema::*;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::domain::repository::user_repository::UserRepository;

///
/// Entity
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Queryable, Insertable)]
#[table_name = "users"]
pub struct UserEntity {
    pub id: u64,
    pub name: String,
}
impl UserEntity {
    fn from(model: &User) -> Self {
        Self {
            id: model.id as u64,
            name: model.name.clone(),
        }
    }
    fn of(&self) -> Result<User, String> {
        User::new(self.id as usize, self.name.clone())
    }
}

///
/// Repository impl
///
pub struct UserRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<MysqlConnection>>>,
}
impl UserRepositoryImpl {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to database: {}", database_url));

        Self { connection }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn get_user(&self) -> User {
        unimplemented!();
        #[allow(unreachable_code)]
        User::new(1, "".to_owned()).unwrap()
    }
    fn create_user(&self) {
        unimplemented!()
    }
    fn update_user(&self) {
        unimplemented!()
    }
    fn delete_user(&self) {
        unimplemented!()
    }
}
