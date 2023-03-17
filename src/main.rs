mod force;
use force::{element::{units, Element}, Force, force_combat::*};
use units::{*};



fn main() {
    run_combat(
        Force {
            name: String::from("Mechanised Force"),
            forces: vec![
                element_builder(rifleman(), 300, 5),
                element_builder(infantry_fighting_vehicle(), 5, 5)
            ]
        }, 
        Force {
            name: String::from("Goblin Force"),
            forces: vec![
                element_builder(swordsman(), 750, 0),
                element_builder(spearman(), 750, 0),
                element_builder(goblin(), 2000, 0)
            ]
    });
}
