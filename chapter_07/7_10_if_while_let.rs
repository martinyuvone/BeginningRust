fn main(){

    enum E {
        Case1(u32),
        Case2(char),
        Case3(i64, bool),
    }

    let v = E::Case3(1234, true);

/*
    match v {
        E::Case3(n, b) => if b { print!("{}", n) }
        _ => {}
    }
*/

    if let E::Case3(n, b) = v {
        if b { print!("{}", n) }
    }
    println!();
}