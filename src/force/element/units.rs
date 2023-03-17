pub mod weapons;
use weapons::Weapons::{*};

#[derive(Debug, Clone)]
pub struct Unit {
    pub name: String,
    pub hp: i32,
    pub armour: i32,
    pub size: i32,
    pub weapons: Vec<Weapon>,
    pub supply_storage: i32,
}

pub fn swordsman() -> Unit {
    return Unit {
        name: String::from("Swordsman"),
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![sword()],
        supply_storage: 48
    }
}

pub fn spearman() -> Unit {
    return Unit {
        name: String::from("Spearman"),
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![spear()],
        supply_storage: 48
    }
}

pub fn goblin() -> Unit {
    return Unit {
        name: String::from("Goblin"),
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![dagger(), dagger()],
        supply_storage: 48
    }
}

pub fn knight() -> Unit {
    return Unit {
        name: String::from("Knight"),
        hp: 2,
        armour: 3,
        size: 3,
        weapons: vec![sword()],
        supply_storage: 48
    }
}

pub fn cavalry() -> Unit {
    return Unit {
        name: String::from("Cavalry"),
        hp: 4,
        armour: 2,
        size: 4,
        weapons: vec![horse_impact(), cavalry_lance()],
        supply_storage: 48
    }
}

pub fn shortbowman() -> Unit {
    return Unit {
        name: String::from("Shortbowman"),
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![short_bow()],
        supply_storage: 48
    }
}

pub fn longbowman() -> Unit {
    return Unit {
        name: String::from("Longbowman"),
        hp: 2,
        armour: 1,
        size: 2,
        weapons: vec![long_bow()],
        supply_storage: 48
    }
}

// modern day

pub fn rifleman() -> Unit {
    return Unit {
        name: String::from("Rifleman"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![rifle()],
        supply_storage: 96
    }
}

pub fn auto_rifleman() -> Unit {
    return Unit {
        name: String::from("Auto Rifleman"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![automatic_rifle()],
        supply_storage: 120
    }
}

pub fn machine_gunner() -> Unit {
    return Unit {
        name: String::from("Machine Gunner"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![machine_gun()],
        supply_storage: 120
    }
}

pub fn light_at_infantry() -> Unit {
    return Unit {
        name: String::from("ATGM Team"),
        hp: 2,
        armour: 5,
        size: 2,
        weapons: vec![light_atgm()],
        supply_storage: 120
    }
}

pub fn infantry_fighting_vehicle() -> Unit {
    return Unit {
        name: String::from("IFV"),
        hp: 20,
        armour: 30,
        size: 8,
        weapons: vec![heavy_machine_gun()],
        supply_storage: 240
    }
}