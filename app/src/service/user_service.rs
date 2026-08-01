use crate::models::user::User;
use crate::repository::db_sqlite::UserDBSqlite;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
    #[error("internal database error: {0}")]
    Internal(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct UserService {
    db: UserDBSqlite,
}

impl UserService {
    pub fn new(db: UserDBSqlite) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, user: &User) -> Result<(), UserError> {
        self.db.save_user(user).await.map_err(UserError::from)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, UserError> {
        self.db.find_all_users().await.map_err(UserError::from)
    }

    pub async fn delete_user(&self, id: &i32) -> Result<(), UserError> {
        let rows = self.db.delete_user(id).await.map_err(UserError::from)?;
        if rows == 0 {
            return Err(UserError::NotFound);
        }
        Ok(())
    }

    pub async fn update_user_email(&self, id: &i32, email: &str) -> Result<(), UserError> {
        let rows = self.db.update_user_email(id, email).await.map_err(UserError::from)?;
        if rows == 0 {
            return Err(UserError::NotFound);
        }
        Ok(())
    }

    pub async fn find_user_by_username(&self, username: &str) -> Result<User, UserError> {
        self.db.find_user(username).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => UserError::NotFound,
            _ => UserError::Internal(e),
        })
    }

    pub async fn delete_user_by_username(&self, username: &str) -> Result<(), UserError> {
        let rows = self.db.delete_user_by_username(username).await.map_err(UserError::from)?;
        if rows == 0 {
            return Err(UserError::NotFound);
        }
        Ok(())
    }
}
