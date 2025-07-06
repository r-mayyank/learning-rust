fn main () {
    let larger;
    let s1 = String::from("small");
    {
        let s2 = String::from("larger");
        larger = larger(&s1, &s2);
    }
    println!("bigger is: {}", larger);
}

fn larger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}


