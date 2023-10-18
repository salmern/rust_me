fn main() {
    let mut string = String::from("hello world"); //save to the heap
    let str = "welcome to rust"; //string literal - //saved on the stack

    let string_to_str = &str.to_string();

    println!("{}", string_to_str);

    let array_concat_string = ["hello", "world"].concat();

    //format macro
    let combined_string = format!("{} {} {}", string, string_to_str, "for Africa");

    println!("{}", combined_string);

    let add_to_string = &string.push_str("something else");
    let add_another_char = &string.push('d');

    //substring
    let slice = &string[0..4];
    let slice_fourth = &string[0..=4];
    
    println!("{}", slice); //excluded
    println!("{}", slice_fourth); //included.
    println!("{:?}", add_to_string);

    let mut my_name = String::from("idris");
    println!("{}", capitalize_name(&mut my_name));

    println!("second print out {}", my_name)
}

fn capitalize_name(name: &mut String) -> String {
    let first = &name.chars().nth(0).unwrap().to_uppercase();
    let end = &name[1..];
    format!("{}{}", first, end)
}
