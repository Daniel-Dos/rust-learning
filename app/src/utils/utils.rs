use rand::RngExt;

pub fn generate_random_number() -> i32 {
    let mut rng  = rand::rng();
    let numero: u32 = rng.random_range(..10);
    println!("Generated random number: {}", numero);
    numero as i32
}