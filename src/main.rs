
fn is_perm(word1 : &str, word2 : &str) -> bool {
    let mut word1 = word1.chars().collect::<Vec<char>>();
    let mut word2 = word2.chars().collect::<Vec<char>>();

    word1.sort();
    word2.sort();

    word1 == word2
}

fn main() {
       let mut w1 = String::new();
        let mut w2 = String::new();

        println!("Enter the first word: ");
        std::io::stdin().read_line(&mut w1).unwrap();
        let w1 = w1.trim(); 

        println!("Enter the second word: ");
        std::io::stdin().read_line(&mut w2).unwrap();
        let w2 = w2.trim();

        

        if is_perm(&w1, &w2) {
            println!("{} and {} are permutations of each other", w1, w2);
        } else {
            println!("{} and {} are not permutations of each other", w1, w2);
        }
}
