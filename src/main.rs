mod car;
use car::Car;
fn main() {
    println!("Creating car...\n");
    let mut e = Car::new(20.0, 10.0);
    println!("{:#?}\n", e);

    let gas = 4.5;
    println!("Adding {} gallons of gas..\n", gas);
    e.add_gas(gas);
    println!("{:#?}\n", e);

    println!("Filling the tank.\n");
    e.fill_tank();
    println!("{:#?}\n", e);

    let miles = 14.0;
    println!("Driving for {} miles...\n", miles);
    e.drive(miles);
    println!("{:#?}", e);
}
