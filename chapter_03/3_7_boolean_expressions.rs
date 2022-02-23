fn main(){

    let truth = true;
    let falsity = false;

    //not
    println!("{} {}", ! truth, ! falsity);
    //and
    println!("{} {} {} {}", falsity && falsity, falsity && truth, truth && falsity, truth && truth);
    //or
    println!("{} {} {} {}", falsity || falsity, falsity || truth, truth || falsity, truth || truth);
}

