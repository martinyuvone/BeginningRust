fn main(){


    /*
     * This won't work as n is out of scope

     {let n = 10;}
    print!("{}", n);
    
    */

    { 
        let n = 10;
        {
            let m = 4;
            {
                print!("{} ", n);
            }
            print!("{} ", n + m);
        } //End of the scope of m
    }//End of the scope of n

    println!();


    {
        let n = 10;
        {
            let n = 4; //This declaration "shadows" the first one
            print!("{} ",n);
        }//End of the scope of second n
        print!("{} ", n);
    }//End of the scope of third n

    println!();
}
