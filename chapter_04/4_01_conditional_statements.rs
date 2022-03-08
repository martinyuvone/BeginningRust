fn main()
{
    let n = 4;
    if n > 0 { println!("positive"); }


    let x = 0;
    print!("number is");
    if x > 0{
        println!(" positive");
    }else{
        println!(" not positive");
    }

    let m = 4;
    if m > 1000 {
            print!("big");
    }
    else if m > 0 {
            print!("small");
        }
    else if m < 0 {
            print!("negative");
    }
    else {
            print!("neither positive nor negative");
    }

    println!();
}
