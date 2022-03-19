fn main(){

    for n in -3..3 {
        println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative", // Guard
            _ => "plural",
        });
    }
}

    // Guard: Such a clause causes this pattern to match only if that Boolean condition is true. 
    // It is named guard, as it protects the expression by an arbitrary Boolean condition, 
    // in addition to the pattern to match.