#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    age: i32
}

impl User {
    pub fn new(username: String, email: String, age: i32) -> User {
        User {
            username,
            email,
            age
        }
    }
}