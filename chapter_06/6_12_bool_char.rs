fn main(){

    let a: bool = true; print!("[{}]", a);
    let b: char = 'a'; print!("[{}]", b);
    println!();

    let e_grave = 'è';
    let japanese_character = 'さ';
    println!("{} {}", e_grave, japanese_character);

    for n in 32..127 {
        println!("{}: [{}]", n, n as u8 as char);
    }
    for n in 160..256 {
        println!("{}: [{}]", n, n as u8 as char);
    }
}