use std::io;

fn main() {
    println!("Please input which number in the Fibonacci sequence you would like to see.");

    let mut n = String::new();
    io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        
    let n: u32 = n.trim().parse().expect("Please type a number!");

    println!("The {}th number in the Fibonacci sequence is: {}", n, fibonnaci(n));
}

fn fibonnaci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonnaci(n - 1) + fibonnaci(n - 2);
    }
}
