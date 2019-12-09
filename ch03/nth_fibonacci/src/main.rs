use std::io::{self, Write};

fn main() {
    println!("Fibonnaci!");
    print!("Number: ");
    io::stdout().flush().unwrap();

    let mut number = String::new();

    io::stdin().read_line(&mut number)
    .expect("Wrong value!");

    let number: u64 = number.trim().parse()
    .expect("Not a value!");

    println!("Fibonnaci of number {} is {}", number, nth_fibonacci(number));
}


// fn nth_fibo(n: u64) -> u64 {
// 	if n == 2 {
// 		1
// 	} else if n == 1 {
// 		0
// 	} else {
// 		nth_fibo(n-1) + nth_fibo(n-2)
// 	}

// }

fn nth_fibonacci(n: u64) -> u64 {
	let mut last_two: [u64; 2] = [0, 1];

	let mut counter = 2;

	while counter <= n {
		let next_fib = last_two[0] + last_two[1];
		last_two[0] = last_two[1];
		last_two[1] = next_fib;

		counter = counter + 1;
	}

	if n > 1 {
		return last_two[1];
	}

	return last_two[0];
}