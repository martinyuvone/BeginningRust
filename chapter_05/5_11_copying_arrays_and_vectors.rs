fn main(){
 println!("Working with arrays");

 let mut a1 = [4, 56, -2];
 let a2 = [7, 81, 12500];
 println!("{:?} {:?}", a1, a2);

 a1 = a2;
 println!("{:?} {:?}", a1, a2);
 
 a1[1] = 10;
 println!("{:?} {:?}", a1, a2);

 println!("Working with vectors");

 let mut v1 = vec![4, 56, -2];
 let v2 = vec![7, 81, 12500];
 println!("{:?} {:?}", v1, v2);
 
 v1 = v2;
 println!("{:?}", v1);
 
 v1[1] = 10;
 println!("{:?}", v1);


}
