use sqlx::{SqlitePool};
use tracing::{info, warn};
use crate::models::user::User;

pub struct UserDBSqlite {
    db: SqlitePool,
}

impl UserDBSqlite {
    pub async fn init_db() -> Result<Self, sqlx::Error> {
        let db = SqlitePool::connect("sqlite:user-rust.db").await?;
        Ok(Self { db })
    }

    pub async fn save_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT into users (username, email, age) VALUES (?, ?, ?)")
            .bind(user.username())
            .bind(user.email())
            .bind(user.age())
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users= sqlx::query_as::<_, User>("select * from users")
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }

    pub async fn delete_user(&self, id: &i32) -> Result<(), sqlx::Error> {
        let resut = sqlx::query("Delete from users where id = ?")
            .bind(id)
            .execute(&self.db)
            .await?;

        if resut.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    pub async fn update_user_email(&self, id: &i32, email: &str) -> Result<(), sqlx::Error> {
        let result  = sqlx::query("update users set email = ? where id = ?")
            .bind(email)
            .bind(id)
            .execute(&self.db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}