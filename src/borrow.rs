fn main() {
    let mut string = String::from("hello");
    update_string(&mut string);
    println!("{}", string)
}

fn update_string(str: &mut String) {
    str.push_str(" World")
}