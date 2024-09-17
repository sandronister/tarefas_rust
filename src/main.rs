
fn is_even(number_i32:i32) -> bool {
   for i in 2..(number_i32-1) {
         if number_i32 % i == 0 {
              return false;
         }
   }
   return true;
}


fn main() {

   for i in 1..100 {
       if is_even(i) {
           println!("{} is even", i);
       } else {
           println!("{} is odd", i);
       }
   }
}
