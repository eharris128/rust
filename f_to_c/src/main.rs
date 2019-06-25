use std::io;

fn main() {
    println!("Please input a temperature in farenheit!");
    loop {
        let mut temp = String::new();
        println!("initial temp: {}", temp);
        io::stdin().read_line(&mut temp)
            .expect("Something went wrong reading line.");
        let temp: f32 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };
        let temp: f32 = (temp - 32.0) * (0.55555555555);
        println!("temp in celsius: {}", temp);   
        break;
    }
}
