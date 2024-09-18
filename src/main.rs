use core::f32;
use std::io;


fn calculate_average(n: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in n.iter() {
        sum += i;
    }
    let media=sum/n.len() as f32;
    media
}


fn main() {
        let mut n = String::new();
        println!("Enter the score separated by space: ");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: Vec<f32> = n.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
        let sum = calculate_average(&n);
        println!("The media of the score: {}", sum);
}
