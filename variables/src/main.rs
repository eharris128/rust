// stopped @ p35..
fn main() {
    let x = 5;
    println!("The val of x is: {}", x);
    let x = 8;
    println!("The val of x is: {}", x);

    const MAX_VAL: u32  = 1_000_000;
    println!("The val of MAX_VAL is: {}", MAX_VAL);

    // let mut _spaces = "  ";
    // _spaces = _spaces.len();

    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    // Numeric operations
    let _sum = 5 + 10;

    let _diff = 2.2 - 1.1;

    let _prod = 4 * 20;

    let _quotient = 56.7 / 32.3;

    let _remainder = 43 % 5;

    // bool
    let _t = true;

    let _f: bool = false;

    // char
    let _a = 'a'; // single quotes

    // compound types
    
    // tuples
    let tup: (i32, f64, u8) = (1500, 6.2, 2);

    let (_a, _b, c) = tup;

    println!("The value of c is: {}", c);
    
    let fifteen_hundred = tup.0;
    println!("fifteen_hundred: {}", fifteen_hundred);

    let six_point_two = tup.1;

    println!("six_point_two: {}", six_point_two);

    let two = tup.2;

    println!("two: {}", two);

    // arrays
    let _arr = [1, 2, 3];

    let months = ["jan", "feb", "mar"];
    
    let jan = months[0];
    println!("jan: {}", jan);

    // functions
    log_hi();

    log_pos_num(2);

    // invalid function
    // let invalid = (let y  = 6);

    // valid function
    let _n = 2;
    
    let y = {
        let n = 1;
        n + 1 // 1 + 1
    };
    println!("y: {}", y); // logs two

    // funcs with return values
    let ret_four = four();
    println!("ret_four: {}", ret_four); // logs four 

    // control flow
    if 4 != ret_four {
        println!("not four");
    } else if ret_four == 2 {
        println!("ret_four ended up being two?")
    } else {
        println!("ret_four == four");
    }
    
    // invalid conditionals
    // if ret_four {
    //     println!("ret_four is not a bool...");        
    // }

    // if / let statement
    const ALWAYS_TRUE: bool = true;
    let if_let = if ALWAYS_TRUE {
        "words"
    } else {
        "other words.. :)"
    };
    println!("if_let evals to: {}", if_let);

    // invalid if / let 
    // let if_let_two = if ALWAYS_TRUE {
    //     "words"
    // } else {
    //    1234
    // };
    
    // Repetition with loops

    // while loops
    let mut changing_number = 3;
    while changing_number != 0 {
        println!("{}!", changing_number);
        changing_number = changing_number - 1;
    }
    println!("liftoff");

    // loop through collection
    let coll = [1, 2, 3];
    let mut index = 0;

    while index < 3 {
        println!("coll val: {}", coll[index]);
        index += 1;
    }

    // interate through a coll
    for item in coll.iter() {
        println!("item val: {}", item);
    }

    // iterate backwards
    for num in (1..4).rev() {
        println!("num val: {}", num);
    }
    println!("liftoff");

}

fn four() -> i32 {
    4
}
fn log_hi() {
    println!("hi");
}

fn log_pos_num(x: u8) {
    println!("x: {}", x);
}

