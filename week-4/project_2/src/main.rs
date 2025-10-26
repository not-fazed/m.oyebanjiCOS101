//Annual incentive calculator using employee age and experience

use std::io;
fn main() {
   
   let mut input1 = String::new();
   let mut input2 = String::new();

   //Experience
    println!("\nAre you experienced?(Yes/No)");
    io::stdin().read_line(&mut input1).expect("This is not a String");
    let exp = input1.trim().to_lowercase();

    //Age
    println!("\nHow old are you?");
    io::stdin().read_line(&mut input2).expect("This is not a String");
    let age:u32 = input2.trim().parse().expect("This is not a number");

    //Incentive
    let mut incentive:u32 = 0;

    if exp =="yes"{
      if age >= 40 {
         incentive = 1_560_000;
      } else if age >= 30 && age < 40{
         incentive = 1_480_000;
      } else if age < 28 {
         incentive = 1_300_000;
      }
    }else {
      incentive = 100_000;
    }
   
   println!("\nEmployee incentive Report");
   println!("\nYour incentive: {}", incentive);
}