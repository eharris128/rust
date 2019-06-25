use std::io;

fn main() {
    println!("Please input a positive number (n) to find the nth Fibonacci number!");
    loop {
        let mut num = String::new();
        println!("initial number: {}", num);
        io::stdin().read_line(&mut num)
            .expect("Something went wrong reading line.");

        // receives a positive number n
        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let num: u32 = calculate_fib(num);

        // outputs the nth fibb number
        println!("nth fib number is: {}", num);   
        break;
    }
}

fn calculate_fib(x: u32) -> u32 {
    let mut tmp_val: u32;
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for _i in 1..x {
        tmp_val = a;

        a = a + b;

        b = tmp_val;
    }
    b
}