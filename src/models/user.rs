#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
}

impl User {
    pub fn new(id: u32, name: String) -> User {
        User { id, name }
    }
}
