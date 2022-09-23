enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Sparkling"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Fruity => println!("Fruity"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0
    };
    print_drink(sweet);

    let fluity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 10.0
    };
    print_drink(fluity);
}