pub mod kmp{
    struct KMP {
        text: String,
        target: String,
        found: bool,
    }
    
    // KMP ALGORITHM FUNCTION
    pub fn kmp_search(word: &String, pattern: &String){
        if word.is_empty() || pattern.is_empty(){
            panic!("Text or pattern was empty");
        }
    
        println!("\nText: {}\nTarget Word: {}", word, pattern);
    
        let mut jump = vec![0; pattern.chars().count()];
        jump[0] = 0;
        jump[1] = 0;    
        
        let word: Vec<char> = word.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();
    
        let mut count = 2;
        let mut jr = 0;
    
        // Compute the jump / match table
        while count < pattern.len() {
            if pattern[count - 1] == pattern[jr] {
                jr += 1;
                jump[count] = jr;
                count += 1;
            }
    
            else if jr > 0 {
                jr = jump[jr];
            }
    
            else {
                jump[count] = 0;
                count += 1;
            }
        }
    
        jr = 0;
        count = 0;
    
        // Search for pattern
        while count < word.len(){
            if jr == pattern.len() - 1{
                println!("Matching word has been found on character {} up to {}", count - jr + 1, count);
    
                return;
            }
    
            // Character matches
            if word[count] == pattern[jr]{
                count += 1;
                jr += 1;
            }
    
            else {
                if jr != 0 {
                    jr = jump[jr];
                }
    
                else {
                    count += 1;
                }
            }
        }
    
        println!("Pattern was not found in the text");
    }
}
