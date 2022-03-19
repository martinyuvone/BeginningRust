fn main(){

    /* Here the compiler deduces that the variable i is used as an array index */
    let a = ["Hello World"];
    let i = 0;
    print!("{}", a[i]);
    println!();

    let x = 0;
    let _j: u16 = x;
    let _k = x;

    /*  In the final stage of every successful compilation, 
        every variable has one concrete, constrained type. */
}