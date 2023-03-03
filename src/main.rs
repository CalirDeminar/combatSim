mod force;
use force::{element::{units::{swordsman, spearman}, Element}, Force};

fn run_round(force_a: Force, force_b: Force) {
    println!("{:?}", force_a);
    println!("{:?}", force_b);
}

fn main() {
    let swordsman_unit = swordsman();
    let spearman_unit = spearman();
    let force_a = Force {
        name: String::from("Force A"),
        forces: vec![Element {unit_type: swordsman_unit, count: 100}]
    };
    let force_b = Force {
        name: String::from("Force B"),
        forces: vec![Element {unit_type: spearman_unit, count: 100}]
    };
    run_round(force_a, force_b);
}
