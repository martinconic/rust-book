use std::io::{self, Write};

fn main() {
    println!(" --- Fahrenheit to Celsius ---");
    print!("Fahrenheit: ");
    io::stdout().flush().unwrap();

    let mut fahr = String::new();

    io::stdin().read_line(&mut fahr)
    	.expect("Wrong value!");

    let fahr: f32 = fahr.trim().parse()
    .expect("Not a valid temperature value!");	


    println!("Celsius: {}", (fahr - 32.0) / 1.8);	 
}
