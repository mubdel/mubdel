use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::Rng;

pub fn generate_session() -> String {
    OsRng
        .sample_iter(Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

pub fn generate_checkout_session() -> String {
    OsRng
        .sample_iter(Alphanumeric)
        .take(35)
        .map(char::from)
        .collect()
}
