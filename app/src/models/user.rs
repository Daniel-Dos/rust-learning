#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    age: i32
}

impl User {
    pub fn new(username: String, email: String, age: i32) -> User {
        User {
            id: 0,
            username,
            email,
            age
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn age(&self) -> &i32 {
        &self.age
    }
    pub fn id(&self) -> &i32 { &self.id }
}