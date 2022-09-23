enum Direction {
    Left,
    Right,
    Up,
}

struct GroceryItem {
    stock: i32,
    price: f64
}

fn main() {
    println!("Hello, world!");

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

}
