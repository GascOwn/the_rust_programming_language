pub fn variables(){

    // variables are immutable by default, they can be made mutable with the mut keyword
    let x = 5; 
    println!("x is {}", x);

    let mut y = 9;
    println!("y is {}", y);

    y = 32;
    println!("after reassignment, y is {}", y);

    // trying to change x's value would give a compiler error 
}
