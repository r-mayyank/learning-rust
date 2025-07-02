fn main() {
    let is_male = true;
    let is_legal = true;

    if is_male {
        println!("you are a male");
    } else {
        println!("you are not a male");
    }

    if is_male && is_legal {
        println!("you are a legal male");
    }

    let greeting = String::from("hello world");

    let sum : i32 = so_sum(5, 10);
    println!("The sum is: {}", sum);

}

fn so_sum(a: i32, b: i32) -> i32 {
    return a +b;
}
