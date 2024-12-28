fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // None
    // Correct way to access elements if you need to use index after iterating.
    if vec.len() > 0 {
        println!("{:?}", vec[0]);
    }
    // Safer way using get() to avoid panics
    match vec.get(1) {
        Some(value) => println!("{:?}", value),
        None => println!("Index out of bounds")
    }
} 