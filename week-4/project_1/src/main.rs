// Roots of the quadratic equation of a,b and c

 use std::io;

fn main () {

    println!("\nEnter value a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let a:f32 = input1.trim().parse().expect("Failed to read input");

    println!("\nEnter value b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let b:f32 = input2.trim().parse().expect("Failed to read input");

    println!("\nEnter value c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let c:f32 = input3.trim().parse().expect("Failed to read input");


    //Calculating Discriminant
    let discriminant = b.powf(2.0) - (4.0*a*c);
    println!("Your discriminant is {}", discriminant);

    if discriminant == 0.0 {
        println!("There is one real root");

    } else if discriminant < 0.0 {
        println!("There is no real root");

    } else if discriminant > 0.0 {
        println!("There is two real roots");  
    }

}