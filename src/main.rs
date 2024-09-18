use core::f32;
use std::io;


fn real_sum(n: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in n.iter() {
        sum += i;
    }
    sum
}


fn main() {
        let mut n = String::new();
        println!("Enter the numbers separated by space: ");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: Vec<f32> = n.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
        let sum = real_sum(&n);
        println!("The sum of the numbers is: {}", sum);
}
