struct Customer {
    age: i32
}

fn try_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        Err("customer must be at least 21 years old".to_owned())
    } else {
        Ok(())
    }
}

fn main() {
    let ashley = Customer{age: 20};
    let purchased = try_purchase(&ashley);
    println!("{:?}", purchased);
}