/* In other words, the index of an array or vector should be unsigned, and it should
have the same size of a memory address. 

Notice that it is not relevant on which system the compiler runs, but on which
system the program generated by the compiler will run. Actually, by a so-called cross-­
compilation, a compiler can generate machine code for a system having a different
architecture from the one where the compiler is run. 

So, there is a need to specify an integer numeric type having a size dependent on the 
target, which is a 16-bit integer if the target is a 16-bit system, a 32-bit integer if 
the target is a 32-bit system, and a 64-bit integer if the target is a 64-bit system.

To such purpose, Rust contains the isize type and the usize type.
*/

fn main(){

    let arr = [11, 22, 33];
    let i: usize = 2;
    print!("{}", arr[i]);

    println!();
}
