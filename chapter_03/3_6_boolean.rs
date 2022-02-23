fn main(){

    let truth = true;
    let falsity = false;

    println!("{} {}", truth, falsity);

    let truth2 = 5 > 2; //true
    let falsity2 = -12.3 >= 10.; //false

    println!("5 > 2 is {}, -12.3 >= 10 is  {}, and -50 < 6 is {}", truth2, falsity2, -50 < 6 );

    // Lexicon comparison
    
    println!(" abc < abcd is {}, ab < ac is {}, and A < a is {}", "abc" < "abcd", "ab" < "ac", "A" < "a");


}
