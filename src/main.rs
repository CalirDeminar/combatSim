mod force;
use force::{element::{units, Element}, Force, CombatLog, force_combat::*};
use units::{swordsman, spearman};

fn main() {
    let force_a = Force {
        name: String::from("Force A"),
        forces: vec![
            Element {unit_type: swordsman(), count: 50},
            Element {unit_type: spearman(), count: 50}
            ]
    };
    let force_b = Force {
        name: String::from("Force B"),
        forces: vec![Element {unit_type: spearman(), count: 50}, 
        Element {unit_type: swordsman(), count: 50}
        ]
    };
    run_combat(force_a, force_b);
}
