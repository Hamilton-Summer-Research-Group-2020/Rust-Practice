// :: like in c++ to use namespaces / libraries ??
use std::io;

fn main() {
    println!("Enter an integer:");
    // let is to declare a local variable
    // mut is for mutable variables
    // type matters (can probably put "use std::String;" at the top)
    let mut n = String::new(); // n = ""

    // all one line but function calls spread by new line for style
    // input stdin
    io::stdin()
        // read input from the user and put into the String from earlier, n
        // & means that it is a reference
        // you need to write &mut n rather than &n to make it mutable
        .read_line(&mut n)

        // read_line outputs an io::Result type which is either ok or err, and
        // expect outputs an error message if it results in an err
        .expect("Failed to read line.");
    println!("You entered {}", n);

    // u32 is an unsigned 32-bit number
    // trim() removes whitespace
    // parse() parses a string to some kind of number (in this case u32)
    // "1" -> 1
    let n: u32 = n.trim().parse().expect("Please type a number!");
    let mut answer = Vec::new();

    for i in 0..n {
        answer.push(fib(i));
    }

    println!("The first {} Fibonacci numbers are ", n);


    print!("[");
    for (idx, val) in answer.iter().enumerate() {
        if idx < answer.len() - 1 {
            print!("{}, ", val);
        }
        else {
            print!("{}", val);
        }
    }
    println!("]");
}

// Specifying parameter and return types (u32)
// Fn: u32 -> u32
fn fib(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    let mut t = 1;
    for _number in 1..n {
        t = a + b;
        a = b;
        b = t;
    }
    t
}
