fn main(){
    use std::mem;

    // i for integer, these are signed integers
    // i8 can go from -128 to +127
    
    let a: i8 = 5;
    let b: i16 = 5;
    let c: i32 = 5;
    let d: i64 = 5;
    let e: i128 = 5;
    print!("{} {} {} {} {}", a, b, c, d, e);
    println!();   
    println!("Sizes in bytes are: a = {}, b = {}, c = {}, d = {}, e = {}", 
        mem::size_of_val(&a),
        mem::size_of_val(&b),
        mem::size_of_val(&c),
        mem::size_of_val(&d),
        mem::size_of_val(&e)
    );
}