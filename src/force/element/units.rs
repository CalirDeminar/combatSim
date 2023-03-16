pub mod weapons;
use weapons::{*};

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
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![sword()]
    }
}

pub fn spearman() -> Unit {
    return Unit {
        name: String::from("Spearman"),
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![spear()]
    }
}

pub fn goblin() -> Unit {
    return Unit {
        name: String::from("Goblin"),
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![dagger(), dagger()]
    }
}

pub fn knight() -> Unit {
    return Unit {
        name: String::from("Knight"),
        hp: 2,
        armour: 3,
        size: 3,
        weapons: vec![sword()]
    }
}

pub fn cavalry() -> Unit {
    return Unit {
        name: String::from("Cavalry"),
        hp: 4,
        armour: 2,
        size: 4,
        weapons: vec![horse_impact(), cavalry_lance()]
    }
}