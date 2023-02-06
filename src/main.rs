use std::io;
use std::env;

fn main() {
    let args = env::args(); // get the args, [0] = program name self

    // TODO: args implementation

    println!("Please enter your message you want to encrypt");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("couldnt read the line ...");

    // TODO: option to generate one via args or via an input blank 

    println!("Please enter your one time pad password (must be longer than the message)");
    let mut otp = String::new();
    io::stdin().read_line(&mut otp).expect("couldnt read the line");

    let encrypted_message = encrypt(message, otp.clone());
    decrypt(encrypted_message, otp);

}

fn decrypt(encrypted_message: String, key: String) -> String {
    const RADIX: u32 = 10;

    // todo: decrypt the string (-> think they are always 7 length or so? )

    String::new()
}

fn encrypt(message: String, key: String) -> String {
    const RADIX: u32 = 10;

    let mut message_in_binary = String::new();

    // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
    // I only called clone() because this for loop takes ownership
    for character in message.clone().into_bytes() {
        message_in_binary += &format!("0{:b}", character);
    }

    let mut otp_in_binary: String = String::new();

    for character in key.clone().into_bytes() {
        otp_in_binary += &format!("0{:b}", character);
    }


    let mut encrypted_bytes_string: String = String::new();
    for (i, cha) in message_in_binary.chars().enumerate() {
        let i_otp: u32 = otp_in_binary.chars().nth(i).unwrap().to_digit(RADIX).unwrap();
        let i_cha: u32 = cha.to_digit(RADIX).unwrap();
        let mut b1: bool = false;
        let mut b2: bool = false;
        if i_cha == 1 {
            b1 = true;
        }
        if i_otp == 1 {
            b2 = true;
        }

        let mut new_bit = 0;
        if (b1 || b2) && !(b1 && b2) {
            new_bit = 1;
        }

        encrypted_bytes_string += &new_bit.to_string();
    }

    println!("msg: {}", message_in_binary);
    println!("key: {}", otp_in_binary);
    println!("enc: {}", encrypted_bytes_string);

    encrypted_bytes_string
}