mod service;
mod models;
mod utils;

use tracing::info;
use models::user::User as user_model;
use service::db_memory::UserDB as user_db;
use utils::utils::generate_random_number as generate_random_number;


fn main() {
    tracing_subscriber::fmt::init();

    info!("Create user");
    let user1 = user_model::new(
        String::from("Daniel"),
        String::from("dds@test.com"),
        generate_random_number(),
    );

    let user2 = user_model::new(
        String::from("Rustacean"),
        String::from("dds@test.com"),
        generate_random_number(),
    );

    info!("Created user1: {:#?}", user1);
    let mut db = user_db::new();
    db.save_user("user1".to_string(), user1);

    info!("Created user2: {:#?}", user2);
    db.save_user("user2".to_string(), user2);

    info!("get user of key -> user1");
    let user = db.find_user("user1");
    info!("User found: {:#?}", user);

    info!("delete user of key -> user1");
    db.delete_user("user1");

    info!("get all users after delete user1");
    let users = db.find_all();
    info!("Users found: {:#?}", users);

    info!("update user of key -> user2");
    let user2_updated = user_model::new(
        String::from("Rustacean"),
        String::from("rustacean@rust.com"),
        generate_random_number(),
    );
    db.update_user("user2".to_string(), user2_updated);

    info!("get all users after update user2");
    let users = db.find_all();
    info!("Users found: {:#?}", users);
}
