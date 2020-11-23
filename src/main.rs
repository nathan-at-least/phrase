use rand;

fn main() {
    let mut rng = get_cryptographically_secure_rng();
    let entropytarget = 128f64;
    let words = parse_words(include_str!("wordlist.txt"));
    let wentropy = (words.len() as f64).log2();

    let mut prefix = "A1! "; // Capital, number, and symbol for dumb password restrictions.
    let mut entropy = 0f64;

    while entropy < entropytarget {
        use rand::seq::SliceRandom;

        let word = words.choose(&mut rng).unwrap();
        print!("{}{}", prefix, word);
        prefix = " ";
        entropy += wentropy;
    }
}

fn parse_words(words: &str) -> Vec<&str> {
    words.split('\n').filter(|&s| s != "").collect()
}

/// This function simply guards the result of thread_rng as being marked
/// cryptographically secure:
fn get_cryptographically_secure_rng() -> impl rand::CryptoRng + rand::Rng {
    rand::thread_rng()
}
