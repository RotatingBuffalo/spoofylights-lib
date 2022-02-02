use std::fmt::Debug;

#[derive(Debug)]
pub struct Car {
    tank_capacity: f64,
    gas_in_tank: f64,
    miles_driven: f64,
    miles_per_gallon: f64,
}
impl Car {
    pub fn new(tank_capacity: f64, miles_per_gallon: f64) -> Self {
        Car {
            tank_capacity,
            gas_in_tank: 0.0,
            miles_driven: 0.0,
            miles_per_gallon,
        }
    }
    pub fn add_gas(&mut self, amount: f64) {
        self.gas_in_tank += amount;
        if self.gas_in_tank > self.tank_capacity {
            self.gas_in_tank = self.tank_capacity;
        }
    }
    pub fn fill_tank(&mut self) {
        self.gas_in_tank = self.tank_capacity;
    }
    pub fn drivable_miles(&self) -> f64 {
        self.gas_in_tank * self.miles_per_gallon
    }
    pub fn drive(&mut self, amount: f64) {
        let gallons_to_use = amount / self.miles_per_gallon;
        if self.drivable_miles() > amount {
            self.miles_driven += amount;
            self.gas_in_tank -= gallons_to_use;
        }
    }
}
