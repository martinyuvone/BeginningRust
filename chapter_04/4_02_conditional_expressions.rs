fn main(){

    let n = 4;
    println!("{}", 
            if n > 1000 {
                "big"
            }
            else if n > 0 {
                "small"
            }
            else if n < 0 {
                "negative"
            }
            else {
                "neither positive or negative"
            }
    );

/* This can be done because the "if" statement
   can be use as an statement and as an expression */

let _a = if true { "abc" } else { "xy" };
let _b = if true { 3456 }  else { 12 };
let _c = if true { 56.9 }  else { 12. }; 

}

