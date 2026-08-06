use crate::models::user::User;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone)]
pub struct UserDBSqlite {
    db: SqlitePool,
}

impl UserDBSqlite {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { db: pool }
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.db
    }

    pub async fn save_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT into users (username, email, age, userid) VALUES (?, ?, ?, ?)")
            .bind(user.username())
            .bind(user.email())
            .bind(user.age())
            .bind(user.user_id())
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn find_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users= sqlx::query_as::<_, User>("select * from users")
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }

    pub async fn find_user(&self, username: &str) -> Result<User, sqlx::Error>{
        let user = sqlx::query_as::<_, User>("select * from users where username = ?")
            .bind(username)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn find_user_by_user_id(&self, user_id: &str) -> Result<User, sqlx::Error>{
        let user = sqlx::query_as::<_, User>("select * from users where userid = ?")
            .bind(user_id)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn delete_user_by_username(&self, username: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE username = ?")
            .bind(username)
            .execute(&self.db)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete_user_by_user_id(&self, user_id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE userid = ?")
            .bind(user_id)
            .execute(&self.db)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete_user(&self, id: &i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("Delete from users where id = ?")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn update_user_email(&self, id: &i32, email: &str) -> Result<u64, sqlx::Error> {
        let result  = sqlx::query("update users set email = ? where id = ?")
            .bind(email)
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn update_user_email_by_username(&self, username: &str, email: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("update users set email = ? where username = ?")
        .bind(email)
        .bind(username)
        .execute(&self.db)
        .await?;
        
        Ok(result.rows_affected())
    }

    pub async fn update_user_email_by_user_id(&self, user_id: &str, email: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("update users set email = ? where userid = ?")
            .bind(email)
            .bind(user_id)
            .execute(&self.db)
            .await?;

        Ok(result.rows_affected())
    }
}
