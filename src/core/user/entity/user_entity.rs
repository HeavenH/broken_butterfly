#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn new(id: i32, first_name: String, last_name: String) -> Self {
        User {
            id,
            first_name,
            last_name
        }
    }
}