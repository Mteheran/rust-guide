fn main() {
    say_hello();
    println!("{}",say_hello_with_return());
    println!("{}",say_hello_with_parameter("hello"))
}

fn say_hello()
{
    println!("hello hello") 
}

fn say_hello_with_return() -> &'static str {
    return "hello hello";
}

fn say_hello_with_parameter(parameter: &'static str) -> String {
    return  format!("hello {}",parameter);
}