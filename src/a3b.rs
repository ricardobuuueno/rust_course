fn main() {
    let n = 7;
    if n > 5 {
        println!(">5")
    } else if n < 5 {
        println!("<5")
    } else {
        println!("=5")
    }

    let my_name = "Bob";
    match my_name {
        "Jayson" => println!("that is my name"),
        "Bob" => println!("not my name"),
        "Alice" => println!("hello alice"),
        _ => println!("that is my name"),
    }
}