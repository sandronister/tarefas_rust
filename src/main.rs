use std::io;


fn table_mult(number_i32: i32) -> [i32; 11] {
    let mut table = [1; 11];
    for i in 0..11 {
        if i != 0 {
            table[i] = number_i32 * (i as i32);
        }
       
    }
    table
}


fn main() {
    println!("Enter a number: ");
    let mut read_number    = String::new();
    io::stdin().read_line(&mut read_number).expect("Failed to read line");
    let number = read_number.trim().parse::<i32>().unwrap();
    let table = table_mult(number);
    for i in 1..11 {
        println!("{} x {} = {}", number, i, table[i]);
    }
   
}
