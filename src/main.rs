mod key_management;
mod communication;

use crate::key_management::generate_keypair;
use crate::communication::{send_message, receive_message};

fn main() {
    // Generate key pairs for Alice and Bob
    let alice_keypair = generate_keypair();
    let bob_keypair = generate_keypair();

    // Alice sends a message to Bob
    let (message, ciphertext) = send_message(&bob_keypair, "Hello, Bob!");

    // Bob receives the message
    let is_valid = receive_message(&bob_keypair, &message, &ciphertext);

    if is_valid {
        println!("Bob successfully received and verified the message: {}", message.content);
    } else {
        println!("Message verification failed.");
    }
}
