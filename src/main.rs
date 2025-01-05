
fn prime_number(number_i32:i32) -> bool {
    if number_i32 < 2 {
        return false;
    }
    for i in 2..number_i32 {
        if number_i32 % i == 0 {
            return false;
        }
    }
    return true;
}


fn main() {

    let max = 10000000;
    for i in 0..max {
        if prime_number(i) {
            println!("{} is a prime number", i);
        }
    }
}
