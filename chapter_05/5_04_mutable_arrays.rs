fn main(){

    let mut x = ["This", "is", "a", "sentence"];
    x[2] = "a nice";
    println!("{} {} {} {}.", x[0], x[1], x[2], x[3]);

    let mut array = ["a", "b", "c"];
    println!("{}{}{}. ", array[0], array[1], array[2]);
    
    array = ["X", "Y", "Z"]; 
    println!("{}{}{}. ", array[0], array[1], array[2]);

    let y = ["1", "2", "3"];
    array = y; 
    println!("{}{}{}. ", array[0], array[1], array[2]);

}

