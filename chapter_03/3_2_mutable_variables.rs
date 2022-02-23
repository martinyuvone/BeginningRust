fn main(){

    // If mut is not added, the variable will be a constant
    let mut number = 12;

    println!("{}", number);

    // The mut indicator allows the variable to change it's value after being initialized
    number = 53;
    
    println!("{}", number);
}
