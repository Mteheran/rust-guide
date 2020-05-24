fn main() {
 
    //implicit type annotation
    let number : u8; //no negatives values
    let number2: i8; //Including negative values
   
    number = 1; //you cannot assign negative values
    number2 = -5; // you can assign negative values
    const CONST_NUMBER: i32 = 0;

    // set 2 variables at the same time
    // the variables are inmutable in this way
    let (number3, number4) = (1, 2);
    //number3 = 2; this is not posible
    let mut number5 = 10; // you can set "mut" to se the variable as mutable
    let number5 = number5 +1; //"Shadowing"
    let number5 = format!("El numero es {}", number5.to_string()); // you can mutate the type of the variable

    println!("mi numero es {} {}", number, number2); // string format generic
    println!("mi numero es {1} {0}", number3, number4); //string format with index
    println!("mi numero es {number5}", number5=number5); //assign alias in string formart
}