fn main(){

    for n in 1..11 {
        println!("{}", n*n);
    }
    println!();

    for m in 1..=10 {
        println!("{}", m*m);
    }
    println!();

    let mut limit = 4;
    for x in 1..limit + 2 {
        limit -= 1;
        print!("{} {}, ", limit, x);
    }
    print!("{}", limit);
    println!();
}

