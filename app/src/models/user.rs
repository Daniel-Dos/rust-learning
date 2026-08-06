use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: i32,
    userid: String,
    username: String,
    email: String,
    age: i32
}

impl User {
    pub fn new(username: String, email: String, age: i32, userid: String) -> User {
        User {
            id: 0,
            userid,
            username,
            email,
            age
        }
    }

    pub fn user_id(&self) -> &String { &self.userid }
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