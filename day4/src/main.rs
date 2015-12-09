extern crate crypto;

use std::env;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn calculate_variant_hash(key: &String, num: i32) -> String {
    let input = format!("{}{}", key, num);
    let mut md5 = Md5::new();
    md5.input_str(&input[..]);
    md5.result_str()
}

fn main() {
    if let Some(key) = env::args().nth(1) {
        println!("Santa's key: {}", key);

        for i in 0..10000000 {
            let hash = calculate_variant_hash(&key, i);
            if hash.starts_with("00000") {
                println!("{} -> {}", i, hash);
            }
        }
    }
}
