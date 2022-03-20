fn main(){
    
    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8; 5],
    }

    let data = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    
    println!("{}, {}, {}, {}",
        data.five_bytes[3], data.integer,
        data.fractional, data.character);
    

    struct OtherData {
        integer: i32,
        fractional: f32,
    }
    
    let mut data2 = OtherData {
        integer: 10,
        fractional: 183.19,
    };

    data2.fractional = 8.2;
    
    println!("{}, {}", data2.fractional, data2.integer);

}