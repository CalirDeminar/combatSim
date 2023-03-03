mod forces_types;
use forces_types::Weapon;
use forces_types::Unit;
use forces_types::Element;
use forces_types::Force;

fn run_round(force_a: Force, force_b: Force) {
    println!("{:?}", force_a);
    println!("{:?}", force_b);
}

fn main() {
    let sword = Weapon {
        pen: 1,
        damage: 2,
        rof: 1
    };
    let spear = Weapon {
        pen: 2,
        damage: 1,
        rof: 1,
    };
    let swordsman = Unit {
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![sword]
    };
    let spearman = Unit {
        hp: 1,
        armour: 1,
        size: 1,
        weapons: vec![spear]
    };
    let force_a = Force {
        name: String::from("Force A"),
        forces: vec![Element {unit_type: swordsman, count: 100}]
    };
    let force_b = Force {
        name: String::from("Force B"),
        forces: vec![Element {unit_type: spearman, count: 100}]
    };
    run_round(force_a, force_b);
    println!("Hello, world!");
}
