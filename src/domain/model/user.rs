use anyhow::{bail, Result};

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(id: String, name: String) -> Result<Self> {
        if name.len() < 3 || 100 <= name.len() {
            bail!("ユーザ名は3文字以上100文字未満である必要があります")
        }

        if id.len() < 3 || 100 <= id.len() {
            bail!("ユーザIDは3文字以上100文字未満である必要があります")
        }

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

#[derive(Debug)]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    pub fn new(name: String) -> Result<Self> {
        if name.len() < 3 {
            bail!("ユーザ名は3文字以上である必要があります")
        } else {
            Ok(Self { name })
        }
    }
}
