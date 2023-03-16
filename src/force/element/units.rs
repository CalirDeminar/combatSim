pub mod weapons;
use weapons::{Weapon, sword, spear};

#[derive(Debug, Clone)]
pub struct Unit {
    pub name: String,
    pub hp: i32,
    pub armour: i32,
    pub size: i32,
    pub weapons: Vec<Weapon>,
}

pub fn swordsman() -> Unit {
    return Unit {
        name: String::from("Swordsman"),
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![sword()]
    }
}

pub fn spearman() -> Unit {
    return Unit {
        name: String::from("Spearman"),
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![spear()]
    }
}