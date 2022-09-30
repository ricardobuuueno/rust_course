use chrono::prelude::*;
use thiserror::Error;

enum Direction {
    Left,
    Right,
    Up,
}

struct GroceryItem {
    stock: i32,
    price: f64
}

fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

#[derive(Debug, Error)]
enum ProgramError {
    #[error("menu error")]
    Menu(#[from] MenuError),

    #[error("math error")]
    Math(#[from] MathError),

}

#[derive(Debug, Error)]
enum MenuError {
    #[error("menu item not found")]
    NotFound,
}

#[derive(Debug, Error)]
enum MathError {
    #[error("divide by zero error")]
    DivideByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError> {
    match choice {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => Err(MenuError::NotFound),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(MathError::DivideByZero)
    }
}

fn run(step: i32) -> Result<(), ProgramError> {
    if step == 1 {
        pick_menu("4")?;
    } else if step == 2 {
        divide(1, 0)?;
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");

    let local: DateTime<Local> = Local::now();
    println!("{:?}", local);

    let mut go = Direction::Left;

    match go {
        Direction::Left => println!("Go Left"),
        Direction::Right => println!("Go Right"),
        Direction::Up => println!("Go Up"),
    }

    go = Direction::Right;
    go = Direction::Up;

    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);

    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = coord;
    println!("{:?}, {:?}", x, y);

    let user_info = ("Emma", 20);
    println!("{:?}, {:?}", user_info.0, user_info.1);

    println!("{:?}", run(1));
    println!("{:?}", run(2));

}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_lower() {
        let result = clamp(10, 100, 1000);
        let expected = 100;
        assert_eq!(result, expected, "should be 100");
    }

    #[test]
    fn clamp_upper() {
        let result = clamp(5000, 100, 1000);
        let expected = 1000;
        assert_eq!(result, expected, "should be 1000");
    }

    #[test]
    fn check_div() {
        let result = div(1,1);
        let expected = Some(1);
        assert_eq!(result, expected, "should be 1");
    }

    #[test]
    fn check_concat() {
        let result = concat("a", "b");
        let expected = String::from("ab");
        assert_eq!(result, expected, "Should be equal");
    }
}