use log::error;
use stego_wps::{decode, encode};

fn main() {
    // Initialize logging, e.g., with env_logger
    env_logger::init();

    // Example usage
    let txt = "This is a sentence. This is another sentence.";
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ ";

    match encode(txt) {
        Ok(encoded) => {
            println!("Encoded: {:?}", encoded);
            let decoded = decode(&encoded, character_set);
            println!("Decoded: {}", decoded.unwrap());
        }
        Err(e) => error!("Error: {}", e),
    }
}
