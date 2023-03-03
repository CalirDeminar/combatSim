mod weapons;
use weapons::weapons::{Weapon, sword, spear};

#[derive(Debug)]
pub struct Unit {
    pub hp: i32,
    pub armour: i32,
    pub size: i32,
    pub weapons: Vec<Weapon>,
}

pub fn swordsman() -> Unit {
    return Unit {
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![sword()]
    }
}

pub fn spearman() -> Unit {
    return Unit {
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![spear()]
    }
}