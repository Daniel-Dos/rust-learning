use crate::models::user::User;
use crate::repository::db_sqlite::UserDBSqlite;

pub struct UserService {
    db: UserDBSqlite,
}

impl UserService {
    pub fn new(db: UserDBSqlite) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        self.db.save_user(user).await?;
        Ok(())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = self.db.find_all_users().await?;
        Ok(users)
    }

    pub async fn delete_user(&self, id: &i32) -> Result<(), sqlx::Error> {
        self.db.delete_user(id).await?;
        Ok(())
    }
    pub async fn update_user_email(&self, id: &i32, email: &str) -> Result<(), sqlx::Error> {
        self.db.update_user_email(id, email).await?;
        Ok(())
    }
}