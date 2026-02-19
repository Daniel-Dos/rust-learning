use rand::distr::Alphabetic;
use rand::RngExt;
use tracing::info;

pub fn generate_random_number() -> i32 {
    let mut rng  = rand::rng();
    let numero: i32 = rng.random_range(0..10);
    numero
}

pub fn generate_random_username() -> String {
    let rng  = rand::rng();
    let username_aleatorio = rng.sample_iter(&Alphabetic)
                                       .take(10)
                                       .map(char::from)
                                       .collect();
    username_aleatorio
}

pub fn generate_random_email() -> String {
    let rng  = rand::rng();
    let email_aleatorio: String = rng.sample_iter(&Alphabetic)
                                       .take(10)
                                       .map(char::from)
                                       .collect();
    format!("{}@example.com", email_aleatorio)
}