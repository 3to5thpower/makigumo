use crate::domain::model::user::User;
use crate::infrastructures::database::schema::*;
use anyhow::Result;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::QueryDsl;

use crate::domain::repository::user_repository::UserRepository;

///
/// Entity
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Queryable, Insertable)]
#[table_name = "users"]
pub struct UserEntity {
    pub id: String,
    pub name: String,
}

impl UserEntity {
    fn from(model: &User) -> Self {
        Self {
            id: model.id.clone(),
            name: model.name.clone(),
        }
    }
    fn of(self) -> Result<User> {
        User::new(self.id, self.name)
    }
}

///
/// Repository impl
///
pub struct UserRepositoryImpl {
    pub connection: MysqlConnection,
}

impl UserRepository for UserRepositoryImpl {
    fn get_user(&self, user_id: &str) -> Result<User> {
        use super::super::database::schema::users::dsl;

        let result = dsl::users
            .find(user_id)
            .first::<UserEntity>(&self.connection)?;

        // database内のuserは3文字以上が確定しているためunwrap
        Ok(result.of().unwrap())
    }

    fn get_users(&self) -> Result<Vec<User>> {
        use super::super::database::schema::users::dsl;

        let results = dsl::users.load::<UserEntity>(&self.connection)?;

        let users = results
            .into_iter()
            .map(|user| user.of().unwrap())
            .collect::<Vec<User>>();
        Ok(users)
    }

    fn create_user(&self, user: &User) -> Result<User> {
        let user_entity = UserEntity::from(user);

        diesel::insert_into(users::table)
            .values(user_entity)
            .execute(&self.connection)?;

        self.get_user(&user.id)
    }

    fn update_user(&self, user: &User) -> Result<User> {
        use super::super::database::schema::users::dsl;

        let user_entity = UserEntity::from(user);

        diesel::update(dsl::users.find(user_entity.id.clone()))
            .set((dsl::id.eq(user_entity.id), dsl::name.eq(user_entity.name)))
            .execute(&self.connection)?;

        self.get_user(&user.id)
    }

    fn delete_user(&self, user: &User) -> Result<()> {
        use super::super::database::schema::users::dsl;

        let user_entity = UserEntity::from(user);

        let _ = diesel::delete(dsl::users.find(user_entity.id)).execute(&self.connection)?;
        Ok(())
    }
}
