fn main() {
   let x =5;
   let y = {
       let x = 3;  // it is a 声明
       x + 1
   };
   let condition = true;
   let number = if condition {
       5
   } else {
       6
   };
   println!("the value of numberr is: {}", number);
   let z = plus_one(9);
   println!("z value is: {}", z);

   println!("the value of y is: {}" ,y );
}

fn plus_one(x: i32) ->i32 {
    x +1   // 不带分号是表达式 
}



