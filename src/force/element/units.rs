pub mod weapons;
use weapons::Weapons::{*};

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

pub fn shortbowman() -> Unit {
    return Unit {
        name: String::from("Shortbowman"),
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![short_bow()]
    }
}

pub fn longbowman() -> Unit {
    return Unit {
        name: String::from("Longbowman"),
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![long_bow()]
    }
}

// modern day

pub fn rifleman() -> Unit {
    return Unit {
        name: String::from("Rifleman"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![rifle()]
    }
}

pub fn auto_rifleman() -> Unit {
    return Unit {
        name: String::from("Auto Rifleman"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![automatic_rifle()]
    }
}

pub fn machine_gunner() -> Unit {
    return Unit {
        name: String::from("Machine Gunner"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![machine_gun()]
    }
}

pub fn light_at_infantry() -> Unit {
    return Unit {
        name: String::from("ATGM Team"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![light_atgm()]
    }
}

pub fn infantry_fighting_vehicle() -> Unit {
    return Unit {
        name: String::from("IFV"),
        hp: 20,
        armour: 30,
        size: 8,
        weapons: vec![heavy_machine_gun()]
    }
}