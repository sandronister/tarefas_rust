
fn get_major(list :&Vec<i32>) -> i32 {
    let mut major = list[0];
     
     for item in list.iter().skip(1) {
        if item > &major{
            major = *item;
        }
     }
    major
}


fn main() {

    let list = vec![1, 2, 3, 9, 5, 5, 5, 5, 5, 8, 5, 5, 5];
    let major = get_major(&list);
    println!("Majority element is: {}", major);
}
