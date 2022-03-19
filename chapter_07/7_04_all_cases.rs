fn main(){

    #[allow(dead_code)]
    enum CardinalPoint { North, South, West, East }
    
    let direction = CardinalPoint::South;
    
    /* When using match, every posible value of the enum must be covered */

    match direction {
        CardinalPoint::North => print!("NORTH"),
        CardinalPoint::South => print!("SOUTH"),
        _ => {}, // this avoids a compilation error
    }
}