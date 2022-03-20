fn main(){
    
    let data = (10000000, 183.19, 'Q');
    let copy_of_data = data;
    println!("{}, {}, {}",
    data.0, copy_of_data.1, data.2);

    let data2: (i32, f64, char) = (10000000, 183.19, 'Q');

    let mut data3 = (10000000, 183.19, 'Q');
    data3.0 = -5;
    data3.2 = 'x';
    println!("{}, {}, {}", data3.0, data3.1, data3.2);
}