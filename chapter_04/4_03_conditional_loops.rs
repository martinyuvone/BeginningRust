fn main(){

    println!("First Exercise");

    let mut n = 1;
    while n <= 10 {
        println!("{} ", n * n);
        n += 1;
    }

    println!("Second Exercise");

    let mut x = 0;
    while x < 50 {
        x += 1;
        if x % 3 != 0 {
            if x * x <= 400 {
                println!("{}",x * x);
            }
        }
    }

    println!("Third Exercise");

    let mut i = 0;
    while i < 50 {
        i += 1;
        if i % 3 == 0 { continue; }
        if i * i > 400 { break; }
        println!("{} ", i * i);
    }
}
