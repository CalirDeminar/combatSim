mod force;
use force::{element::{units, Element}, Force, force_combat::*};
use units::{*};



fn main() {
    run_combat(
        Force {
            name: String::from("Mechanised Force"),
            forces: vec![
                element_builder(rifleman(), 200, 5),
                element_builder(infantry_fighting_vehicle(), 6, 5)
            ],
            supply_used: 0
        }, 
        Force {
            name: String::from("Goblin Force"),
            forces: vec![
                element_builder(swordsman(), 750, 0),
                element_builder(spearman(), 750, 0),
                element_builder(goblin(), 2000, 0)
            ],
            supply_used: 0
    });
}
