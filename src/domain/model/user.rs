#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
}

impl User {
    pub fn new(id: usize, name: String) -> Result<Self, String> {
        if name.len() < 3 {
            Err(String::from("ユーザ名は3文字以上である必要があります"))
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
