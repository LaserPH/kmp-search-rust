mod algorithms;
use std::io::{self, Read};
use std::fs::File;  
pub use algorithms::searchers::*;

fn main() {
    println!("KMP STRING SEARCH WRITTEN IN RUST");
    println!("BY: LaserPH\n");

    // Variables
    let mut word = String::new();
    let mut target = String::new();
    let mut mode = String::new();

    // Mode Selection
    println!("Select Mode: ");
    println!("Enter 0 | File Reading Mode");
    println!("Enter 1 | User Input Mode");

    io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read mode");

    mode.pop(); 

    let mode = mode.trim().parse::<u32>().unwrap();

    // File reading mode
    if mode == 0 {
        let mut f = File::open("bears.txt").unwrap();
        
        match f.read_to_string(&mut word) {
            Ok(_) => (),
            Err(e) => panic!("Failed to parse text {:?}", e)
        }

        println!("Enter a target word or pattern: ");
        io::stdin()
                .read_line(&mut target)
                .expect("Failed to read target word or pattern");

        target.pop();

        kmp::kmp_search(&word, &target);
    }

    // User input mode
    else if mode == 1 {
        println!("Enter text:");
        
        // Parse word
        io::stdin()
                .read_line(&mut word)
                .expect("Failed to read word");

        // Remove any new lines
        word.pop();

        println!("Enter a target word or pattern:");

        // Parse target word or pattern
        io::stdin()
                .read_line(&mut target)
                .expect("Failed to read target word or pattern");

        target.pop();

        kmp::kmp_search(&word, &target);
    }
}



