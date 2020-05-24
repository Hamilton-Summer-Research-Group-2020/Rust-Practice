use std::io;

// Function from int -> bool
// If the input is a prime number, return true, otherwise false
fn is_prime(num: u64) -> bool {
    if num == 1 {
        return false;
    }

    // convert the integer input to float, take the sqrt, and then find the floor of it
    let bound = ((num as f64).sqrt()) as u64;

    for i in 2..(bound + 1) {
        if num % i == 0 {
            // println!("Number is divisible by {}", i);
            return false;
        }
    }
    true
}

// [2, 3, 5], 2 -> 5
fn find_prime(num: u64) -> u64 {
    let mut count = 0;
    let mut i = 1;
    while count < num {
        if is_prime(i) {
            count += 1;
        }
        i += 1;
    }
    i - 1
}

fn main() {
    // Asks for input
    println!("Enter a number: ");

    // We will put the input into n
    let mut n = String::new();
    // Read the line and output an error message in the string if not a number
    io::stdin().read_line(&mut n).expect("Please put a number");
    // Clean up the string so that it is a f64 type
    let n: u64 = n.trim().parse().expect("Bro not a number");

    println!("{}th prime is {}", n, find_prime(n))
    println!("{}", is_prime(n));

    // Loop of the program
    // loop {
    //     println!("Enter a number: ");
    //
    //     let mut n = String::new();
    //     io::stdin().read_line(&mut n).expect("Please put a number");
    //     let n: u64 = n.trim().parse().expect("Bro not a number");
    //
    //     println!("{}th prime is {}", n, find_prime(n))
    // }

}
