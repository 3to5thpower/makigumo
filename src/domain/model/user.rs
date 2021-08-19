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
            id.len() < 3 || 100 <= id.len(),
            UserModelError::UserIdLengthError(id)
        );
        ensure!(
            name.len() < 3 || 100 <= name.len(),
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
