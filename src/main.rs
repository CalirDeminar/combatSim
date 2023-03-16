mod force;
use force::{element::{units, Element}, Force, force_combat::*};
use units::{*};

fn main() {
    let force_a = Force {
        name: String::from("Force A"),
        forces: vec![
            Element {unit_type: swordsman(), count: 50},
            Element {unit_type: spearman(), count: 50},
            Element {unit_type: goblin(), count: 100}
            ]
    };
    let force_b = Force {
        name: String::from("Force B"),
        forces: vec![
            Element {unit_type: knight(), count: 50}, 
            Element {unit_type: cavalry(), count: 25}
        ]
    };
    run_combat(force_a, force_b);
}
