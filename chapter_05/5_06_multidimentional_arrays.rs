fn main() {
    let mut x = [[[23; 4]; 8]; 15];
    x[14][7][3] = 56;

    println!("{}, {}", x[0][0][0], x[14][7][3]);

    // Array sizes
    println!("x size is: {} of {} of {}", x.len(), x[0].len(), x[0][0].len());

}
