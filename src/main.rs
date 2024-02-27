fn main() {
    let s1 = gives_ownership();
    println!("{} world!", s1);
}

fn gives_ownership()-> String{
    let some_string = String::from("hello");
    some_string
}