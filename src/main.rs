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

    io::stdin().read_line(&mut mode)
               .expect("Failed to read mode");

    mode.pop(); 

    // Perform shadowing on the "mode" variable
    let mode = mode.trim().parse::<u32>().unwrap(); 

    let mode_enum: kmp::KMPMode = if mode == 0{ 
        kmp::KMPMode::FileRead 
    } else if mode == 1{ 
        kmp::KMPMode::UserInput 
    } else {
        kmp::KMPMode::NoMode
    };


    match mode_enum {
        kmp::KMPMode::FileRead => {
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

        kmp::KMPMode::UserInput => {
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

        kmp::KMPMode::NoMode => {
            eprintln!("You have entered an invalid input for the mode selection.");
            main();
        }
    }
}



