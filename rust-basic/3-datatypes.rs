fn main() {
   
  //integer types from i8, i16, i32, i64, i138, isize
   let number : u32;
   let number2: i8;

   //float type f32, or f64;
   let float_number64 = 2.1; // float64 by default
   let float_number : f32;

   //string
   let mut mystring: String;
   let other_string: &str = "
   Hello
   hello"; //string in multiples lines
   //let other_string = "Hola"; shadowing 

   // If you need quotes in a raw string, add a pair of #s
   let quotes = r#"And then I said: "There is no escape!""#;
   println!("{}", quotes);
    
   // If you need "# in your string, just use more #s in the delimiter.
   // There is no limit for the number of #s you can use.
   let longer_delimiter = r###"A string with "# in it. And even "##!"###;
   println!("{}", longer_delimiter);

   //char 
   let c = 'c';
   let z : char;
   let heart_eyed_cat = '😻';

   //bool
   let isfalse : bool = false;
   let mut istrue = true;


}