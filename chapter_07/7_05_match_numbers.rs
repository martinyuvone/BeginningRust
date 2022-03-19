fn main(){
    
    match "value" {
        "val" => print!("value "),
        _ => print!("other "),
    }
    match 3 {
        3 => print!("three "),
        4 => print!("four "),
        5 => print!("five "),
        _ => print!("other "),
    }
    match '.' {
        ':' => print!("colon "),
        '.' => print!("point "),
        _ => print!("other "),
    }

    /*Also for match statements with arguments that aren’t enums, it is required that all
      possible cases are handled. However, except for enums and Booleans, it is not 
      feasible to specify all single cases; therefore, it is required to use the 
      underscore “catch-all” case. */
}