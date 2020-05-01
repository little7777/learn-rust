//use rand::Rng;

fn mainloop() {
    // println!("Hello, world!");
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10  {
            break counter *2;
        }
    };
    println!("the result is {}", result);
}

// while loop
fn mainwhile(){
    let mut number =3;
    while number != 0 {
        println!("{}!", number);
        number -=1;
    }
    println!("leave!!!");
}

fn mainfor() {
    let a = [10,20,30,40,50];

    for ele in a.iter() {
        println!("the value is {}:", ele);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("leave out");
}

// 实现斐波那契序列
fn fib_rust(n: u64) ->u64 {
    let mut a =  0;
    let mut b = 1;
    // let secret_number = rand::thread_rng().gen_range(1,n+1) 
    for i in 1..n+1 {
        let tem = a + b;
        a = b;
        b = tem;
    };
    return b;
}

fn main() {
    for i in 1..20 {
        println!("b value is : {}", fib_rust(i));
    }
}



