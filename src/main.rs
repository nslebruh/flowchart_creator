use serde::{Deserialize, Serialize};
use std::fs;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
struct Fluid {
    id: String,
    name: String,
    amount: u32,
    temperature: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: String,
    name: String,
    amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct RecipeThings {
    items: Vec<Item>,
    fluids: Vec<Fluid>
}

#[derive(Serialize, Deserialize, Debug)]
struct Machine {
    voltage_tier: String,
    _type: String,

}

#[derive(Serialize, Deserialize, Debug)]
struct GTNHRecipe {
    input: RecipeThings,
    output: RecipeThings,
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
    let data = fs::read_to_string("recipes.json").expect("Failed to read recipes.json");
    println!("{}", data);
    let recipes: Vec<GTNHRecipe> = from_str(&data).expect("Unable to read data");
    println!("{}", recipes[0].circuit)
}