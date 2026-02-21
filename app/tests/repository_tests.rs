use sqlx::SqlitePool;
use app_rust::repository::db_sqlite::UserDBSqlite;
use app_rust::models::user::User;

async fn setup_db(pool: SqlitePool) {
    sqlx::query(r#"CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT,
                       username TEXT NOT NULL, email TEXT NOT NULL,
                       age INTEGER NOT NULL);"#)
        .execute(&pool)
        .await
        .unwrap();
}
#[sqlx::test]
async fn should_save_and_get_user(pool: SqlitePool) {
    setup_db(pool.clone()).await;

    let db = UserDBSqlite::new(pool);

    let user1  = User::new("Teste".to_string(),"teste@teste.com".to_string(), 20);
    db.save_user(&user1).await.unwrap();

    let users = db.find_all_users().await.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].username(), "Teste");
}

#[sqlx::test]
async fn should_delete_user(pool: SqlitePool){
    setup_db(pool.clone()).await;
    let db = UserDBSqlite::new(pool);

    let user1  = User::new("Teste".to_string(),"teste@teste.com".to_string(), 20);
    db.save_user(&user1).await.unwrap();

    let users = db.find_all_users().await.unwrap();
    let user_id = users.first().unwrap().id();

    db.delete_user(&user_id).await.unwrap();
    assert!(db.find_all_users().await.unwrap().is_empty());
}

#[sqlx::test]
async fn should_update_user_email(pool: SqlitePool){
    setup_db(pool.clone()).await;
    let db = UserDBSqlite::new(pool);

    let user1  = User::new("Teste".to_string(),"teste@teste.com".to_string(), 20);
    db.save_user(&user1).await.unwrap();

    let users = db.find_all_users().await.unwrap();
    let user_id = users.first().unwrap().id();
    db.update_user_email(&user_id, "teste.teste@teste.com").await.unwrap();

    let users = db.find_all_users().await.unwrap();
    let user_email = users.first().unwrap().email();
    assert_eq!(user_email, "teste.teste@teste.com");
}