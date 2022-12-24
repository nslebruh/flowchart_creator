use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Fluid {
    id: String,
    name: String,
    amount: u32,
    temperature: i32
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: String,
    name: String,
    amount: u32,
}

#[derive(Serialize, Deserialize)]
struct RecipeThings {
    items: Vec<Item>,
    fluids: Vec<Fluid>
}

#[derive(Serialize, Deserialize)]
struct Machine {
    voltage_tier: String,
    _type: String,

}

#[derive(Serialize, Deserialize)]
struct GTNHRecipe {
    input: Vec<RecipeThings>,
    output: Vec<RecipeThings>,
    machine: Machine,
    ticks: u32,
    eu_per_tick: u32,
    total_eu: u32,
    multiblock: bool,
    requires_cleanroom: bool,
    requires_circuit: bool,
    circuit: u32

}

fn main() {
    println!("Hello, world!");
}
