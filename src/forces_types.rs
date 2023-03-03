#[derive(Debug)]
pub struct Weapon {
    pub pen: i32,
    pub damage: i32,
    pub rof: i32
}

#[derive(Debug)]
pub struct Unit {
    pub hp: i32,
    pub armour: i32,
    pub size: i32,
    pub weapons: Vec<Weapon>,
}

#[derive(Debug)]
pub struct Element {
    pub unit_type: Unit,
    pub count: i32
}

#[derive(Debug)]
pub struct Force {
    pub name: String,
    pub  forces: Vec<Element>,
}