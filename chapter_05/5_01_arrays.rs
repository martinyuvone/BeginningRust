fn main(){

    let x = ["English", "This", "sentence", "a", "in", "is"];
    print!("{} {} {} {} {} {}",
        x[1], x[5], x[3], x[2], x[4], x[0]);
    println!();

    let a = [true, false];
    let b = [1, 2, 3, 4, 5];
    print!("{}, {}.", a.len(), b.len());
    println!();
}
