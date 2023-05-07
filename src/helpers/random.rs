use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn random_string(len: usize) -> String {
    let rng = rand::thread_rng();

    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
