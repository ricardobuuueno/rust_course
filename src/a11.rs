struct GroceryItem {
    quantity: i32,
    id: i32
}

fn display_quantity(item: &GroceryItem) {
    println!("Quantity: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("Id: {:?}", item.id);
}

fn main() {
    let item = GroceryItem {
        quantity: 3,
        id: 99
    };
    display_quantity(&item);
    display_id(&item);
}