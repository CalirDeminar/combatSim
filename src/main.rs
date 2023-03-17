mod force;
use force::{element::{units, Element}, Force, force_combat::*};
use units::{*};



fn main() {
    let goblin_force = Force {
            name: String::from("Goblin Force"),
            forces: vec![
                Element {unit_type: swordsman(), count: 500},
                Element {unit_type: spearman(), count: 500},
                Element {unit_type: goblin(), count: 1000}
                ]
    };
    let knight_cav_force = Force {
        name: String::from("Knight Cav Force"),
        forces: vec![
            Element {unit_type: knight(), count: 500}, 
            Element {unit_type: cavalry(), count: 250}
        ]
    };
    let archer_force = Force {
        name: String::from("Archer Force"),
        forces: vec![
            Element {unit_type: shortbowman(), count: 400}, 
            Element {unit_type: longbowman(), count: 250}
        ]
    };
    run_combat(goblin_force, archer_force);
}
