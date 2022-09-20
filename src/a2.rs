fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_results(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let result = sum(2,2);
    display_results(result);
}