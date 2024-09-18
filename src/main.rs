use std::io;

fn search_char(n: &str) -> bool {
    let mut _chars: Vec<char> = Vec::new();
    for i in n.trim().chars() {
        if _chars.contains(&i) {
            return true;
        }
        _chars.push(i);
    }
    false
}

fn main() {
        let mut n = String::new();
        println!("Write the word ");
        io::stdin().read_line(&mut n).expect("Failed to read line");

        let result = search_char(&n);

        if result {
            println!("The word not contains only characters");
        } else {
            println!("The word not really contains only characters");
        }
       
}
