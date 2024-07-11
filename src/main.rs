fn main() {
    // variable integer
    let x: i32 = -10;
    let positive_integer: u32 = 10;
    let float_number: f32 = 10.0;
    print!("Vikas Yadav");

    print!(": {}", x);
    print!(": {}", positive_integer);
    print!(": {}", float_number);

    // Boolean
    let is_male: bool = true;
    let is_age_less_than_18: bool = true;

    // if else condition
    if is_male {
        println!("Your are a male bro")
    }

    if is_age_less_than_18 {
        println!("You are kid")
    }

    if is_male && is_age_less_than_18 {
        println!("You are kid as well as male")
    }

    // string
    let greeting: String = String::from("String declared using String::from()");
    println!("{}", greeting);
}
