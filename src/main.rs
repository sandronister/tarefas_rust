use std::io;


fn mean(table: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for i in table {
        sum += i;
    }
    sum 
}


fn main() {
    let mut table: Vec<f32> = Vec::new();
    loop {
        println!("Enter a number: ");
        let mut read_number    = String::new();
        
        io::stdin().read_line(&mut read_number).expect("Failed to read line");

        if read_number.trim().is_empty() {
            let total = mean(&table);
            println!("The mean of the numbers is: {}", total);
            break;
        }
            

        let number: f32 = match read_number.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
            
        };
        table.push(number)
    }
   
}
