fn main(){

#[allow(dead_code)]
    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
        }

    let contin = Continent::Asia;

    match contin {
        Continent::Europe => println!("E"),
        Continent::Asia => println!("As"),
        Continent::Africa => println!("Af"),
        Continent::America => println!("Am"),
        Continent::Oceania => println!("O"),
    }
}