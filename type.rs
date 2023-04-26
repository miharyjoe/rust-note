fn main() {
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too

    let my_number = 100; // We didn't write a type of integer,
    // so Rust chooses i32. Rust always
    // chooses i32 for integers if you don't
    // tell it to use a different type

//println!("{}", my_number as char); // ⚠️
println!("{}", my_number as u8 as char);

println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
println!("Size of string containing 'ß': {}", "ß".len());
println!("Size of string containing '国': {}", "国".len());
println!("Size of string containing '𓅱': {}", "𓅱".len());

let slice = "Hello!";
println!("Slice is {} bytes.", slice.len());
println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
let slice2 = "안녕!"; // Korean for "hi"
println!("Slice2 is {} bytes.", slice2.len());
println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());

//TYpe declaration
let small_number: u8 = 10;
let small_number = 10u8; // 10u8 = 10 of type u8
let small_number = 10_u8; // This is easier to read
let big_number = 100_000_000_i32; // 100 million is easy to read with _

println!("{:?}", b"this is a test");

    println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");// Try printing them with unicode escape \u
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right

    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));


    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );


    let my_string: String = "Try to make this a String".into();
}
