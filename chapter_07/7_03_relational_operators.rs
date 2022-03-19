fn main(){
    /* The following code generates a compilation error */

    enum CardinalPoint { North, South, West, East }
    let  direction = CardinalPoint::South;
    
    if direction == CardinalPoint::North { }

}