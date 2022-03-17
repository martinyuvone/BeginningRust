fn main(){

    // Declare a fixed size aray of 5000 floating numbers
    let mut x = [4. ; 5000];
    x[2000] = 3.14;

    println!("{} {}", x[1000], x[2000]);


    // Scan items of an array
    let mut fib = [1; 12];

    for i in 2..fib.len() {
        fib[i] = fib[i - 2] + fib[i - 1];
    }

    for i in 0..fib.len() {
        print!("{}, ", fib[i]);
    }

    println!();
    println!("FIB length is {}", fib.len());
}

