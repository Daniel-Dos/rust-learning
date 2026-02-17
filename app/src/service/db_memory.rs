use std::collections::HashMap;
use crate::models::user::User;
use tracing::{info};


pub struct UserDB {
   db: HashMap<String, User>,
}

impl UserDB {
    pub fn new() -> UserDB {
        UserDB { db: HashMap::new() }
    }

    pub fn save_user(&mut self,key: String ,user: User) {
        self.db.insert(key, user);
       info!("User saved");
    }

   pub fn find_user(&self, key: &str) -> Option<&User> {
       self.db.get(key)
   }

    pub fn delete_user(&mut self, key:&str) {
        self.db.remove(key);
        info!("User deleted");
    }

    pub fn find_all(&self) -> Vec<&User> {
        self.db.values().collect()
    }

    pub fn update_user(&mut self, key:String, user: User) {
        self.db.insert(key, user);
        info!("User updated");
    }
}