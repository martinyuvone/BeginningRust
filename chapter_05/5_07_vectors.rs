fn main(){

    let x = vec!["This", "is"];
    println!("{} {}. Length: {}.", x[0], x[1], x.len());
    println!();

    let mut v = vec!["This", "is"];
    println!("{}", v.len());
    v.push("a");
    println!("{}", v.len());
    v.push("sentence");
    println!("{}", v.len());
    v[0] = "That";

    for i in 0..v.len() { 
        print!(" {}", v[i]);
    }
    println!();


}
