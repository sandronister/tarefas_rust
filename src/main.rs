fn count(num : i32){
    for i in 1..num {
        println!("Contando ...{}", i);
    }
}

fn down(num : i32){
    for i in (1..num).rev() {
        println!("Contando ...{}", i);
    }
}

fn main() {

    count(10);
    down(10);
    
    println!("Hello, world!");
}
