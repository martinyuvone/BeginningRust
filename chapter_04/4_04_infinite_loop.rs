fn main(){

    println!("First Loop");

    let mut n = 1;
    while true{
        let n2 = n * n;
        if n2 >= 200 { break; }
        print!("{} ", n2);
        n += 1;
    }

    println!();
    println!("Socond Loop");

    let mut x = 1;
    loop {
            let x2 = x * x;
                if x2 >= 200 { break; }
                    print!("{} ", x2);
                        x += 1;
    }
    println!();
}
