//! Android Pattern Cracker
//! Author : Nathan CORNELOUP
//! Version : 0.1.0

//! This program allow you to crack Android gesture.key files up to Android 5.1
//! Usage : ./android_pattern_cracker -f gesture.key

//! Exemple of returned pattern : 9852147
//! Means : 9 -> 8 -> 5 -> 2 -> 1 -> 4 -> 7 on the Pattern lockscreen
//! 
//! [1] [2] [3]
//! [4] [5] [6]
//! [7] [8] [9]


use sha1::{Sha1, Digest};
use std::{fs::File, io::Read};
use std::env::{self};
use hex;

static MAX_PATTERN: u64 = 987654321;
static VERSION: &str = "0.1.0";

fn main() {

    // Retrieving console arguments
    let args: Vec<String> = env::args().collect();

    match &args[..] {
        [_,b,c] => {
            if b != "-f"{
                banner();
            }
            else{
                let filename: String = c.to_string();
                crack_target(filename);
            }
        
            }
        ,
        _ => banner()
    }


}

// Print the usage informations.
fn banner() -> (){
    println!("Android Pattern Cracker: v{}", VERSION);
    println!("Usage is : {} -f gesture.key", env::args().collect::<Vec<String>>()[0]);
}

// Crack the designated gesture.key file
fn crack_target(filename: String) -> (){
    let target = process_file(filename);
    println!("Target hash: {:?}",String::from("0x".to_string() + &hex::encode(target).to_owned()));

    match bruteforce(target){
        Some(x) => println!("Pattern found : {}",x),
        None => println!("No pattern found."),
    }
}

// Process the file given
// Filename need to reference a relative path from the program itself to an existing file.
// Return an array representing the hexadecimal value of the SHA-1 key.
fn process_file(filename: String) -> [u8;20] {
    let mut file = File::open(filename.clone())
    .expect(&format!("Failed to open {}.", filename.as_str()));

    let mut content: [u8;20] = [0; 20];

    file.read_exact( &mut content)
    .expect("Failed to read the file or the file is not a valid gesture.key file.");

    println!("Processing key file: {}", filename);
    content
}

// Generate the hexadecimal digest of a number
// The number must be in a String
// The number is represented in a vector of u8 where vec\[i\] is the hexadecimal value of the nth digit of n
// IMPORTANT : The \[1\] button on the Android pattern lock screen is stored as 0, \[2\] as 1, \[3\] as 2, That's why we substract one when calculating the hexadecimal representation.
fn generate_hash(n: String) -> Vec<u8> {
    
    let mut vect_n: Vec<u8> = Vec::new();
    // Decomposition of the number into a vect of number

    for e in n.chars() {
        // Conversion of the char to a u8 - 1 ( because the 1 digit is considered as 0)
        let mut number: u8 = e.to_digit(10).unwrap() as u8;
        if number > 0 {
            number -= 1;
        }
        vect_n.push(number);

    }

    vect_n
    
}

// Compare two hexadecimal hash
// Hash: Vec<u8>, Target: [u8;20]
fn matching_hash(hash: Vec<u8>, target: [u8;20]) -> bool {
    // The function compare to hash
    if hash.len() != target.len(){
        return false;
    }
    else {
        for i in 0..hash.len() {
            if hash[i] != target[i]{
                return false;
            }
        }
        true
    }


}

// Try all possible pattern combinaison
// The maximum pattern is stored in the MAX_PATTERN constant.
// Since each digit can't be reused, the max pattern is 987654321
fn bruteforce(target: [u8;20]) -> Option<u64> {
    let mut found = false;
    let mut n = 1;
    while !found {

        if n > MAX_PATTERN {
            return None;
        }
        else {
        

        // Creation of the hash
        let hash  = generate_hash(n.to_string());

        let mut hasher = Sha1::new();
        hasher.update(&hash);
        let result = hasher.finalize();

        if matching_hash(result.to_vec(), target) {
            found = true;
        }

        
        else {
            n += 1;
        }
    }

        
    }

    // Correcting the pattern
    // Zero's are not proper handled, so we need to add 1 when they are in the result pattern 
    let stringed_n: String = n.to_string();
    let mut result: String = String::new();

    for e in stringed_n.chars(){
        if e.to_digit(10) == Some(0) {
            result+="1";
        }
        else{
            result += e.to_string().as_str();
        }
    }
    

    Some(result.parse::<u64>().unwrap())

}




// Test cases

#[test]
fn small_pattern() {
    let target: [u8;20] = [0x9F,0xDE,0x91,0x88,0xD6,0xDE,0xB3,0x03,0xB2,0x85,0xFE,0xFE,0xFB,0x2E,0x13,0x1E,0x28,0x9C,0x0E,0x28];
    let pattern: u64 = 15769423;
    assert_eq!(bruteforce(target), Some(pattern));
}

#[test]
fn medium_pattern() {
    let target: [u8;20] = [0x14,0xE0,0x4C,0x86,0x18,0xB4,0x15,0xF3,0x44,0x89,0x02,0x0F,0xDA,0x84,0x25,0x01,0x44,0x5B,0x42,0x95];
    let pattern: u64 = 635781942;
    assert_eq!(bruteforce(target), Some(pattern));
}

#[test]
fn max_pattern() {
    let target: [u8;20] = [0x85,0x38,0x22,0xDC,0xEE,0x4C,0x6B,0x59,0xD4,0xA9,0xF0,0xC4,0xCD,0xAF,0x97,0x98,0x9E,0x29,0xC8,0x3A];
    let pattern: u64 = MAX_PATTERN;
    assert_eq!(bruteforce(target), Some(pattern));
}
