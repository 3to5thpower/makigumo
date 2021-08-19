use anyhow::{bail, Result};

#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
}

impl User {
    pub fn new(id: usize, name: String) -> Result<Self> {
        if name.len() < 3 {
            bail!("ユーザ名は3文字以上である必要があります")
        } else {
            Ok(Self { id, name })
        }
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
