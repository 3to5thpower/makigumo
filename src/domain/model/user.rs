use crate::domain::error::UserModelError;
use anyhow::{ensure, Result};

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(id: String, name: String) -> Result<Self> {
        ensure!(
            3 <= id.len() && id.len() < 100,
            UserModelError::UserIdLengthError(id)
        );
        ensure!(
            3 <= name.len() && name.len() < 100,
            UserModelError::UserNameLengthError(name)
        );

        Ok(Self { id, name })
    }

    pub fn change_user_name(&mut self, name: String) {
        self.name = name;
    }
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id
    }
}
