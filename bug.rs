fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // None
    //The following line causes a panic because we try to access an element that is out of bound
    println!("{:?}", vec[2]);
}