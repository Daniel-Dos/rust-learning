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

    pub async fn save_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT into users (username, email, age) VALUES (?, ?, ?)")
            .bind(user.username())
            .bind(user.email())
            .bind(user.age())
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
