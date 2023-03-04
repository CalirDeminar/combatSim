mod force;
use force::{element::{units, Element}, Force};
use units::{swordsman, spearman, Unit, weapons};
use weapons::Weapon;
use rand::Rng;


fn hit_check(_weapon: &Weapon, _target: &Unit) -> bool {
    let mut rng = rand::thread_rng();

    let i: f32 = rng.gen();
    return i < 0.5;
}

fn pen_check(weapon: &Weapon, target: &Unit) -> bool {
    let mut rng = rand::thread_rng();
    let i: f32 = rng.gen();
    let pen_chance = weapon.pen as f32/(target.armour * 2) as f32;
    return i < pen_chance.clamp(0.0, 1.0);
}

fn random_target(force: &Force) -> &Unit {
    let elements = &force.forces;
    let total_forces = elements.into_iter().fold(0, |acc, e| acc + e.count);
    let odds = elements.into_iter().map(|e| (e.count as f32 / total_forces as f32, &e.unit_type));
    let mut i: f32 = rand::thread_rng().gen();
    for entry in odds {
        if i < entry.0 {
            return &entry.1;
        }
        i -= entry.0;
    }

    return &force.forces[0].unit_type;
}

fn run_round(force_a: Force, force_b: Force) {
    println!("{:?}", force_a);
    println!("{:?}", force_b);
    // FA
    for element in force_a.forces {
        let mut hits: Vec<(&Unit, i32)> = vec![];
        let mut pens: Vec<(&Unit, i32)> = vec![];
        // let mut kills: Vec<(&Unit, i32)> = vec![];
        for _i in 1..element.count {
            for weapon in &element.unit_type.weapons {
                let target_type = random_target(&force_b);
                if hit_check(&weapon, &target_type) {
                    let target_record = hits.iter().position(|x| std::ptr::eq(x.0, target_type)  );
                    match target_record {
                        Some(p) => hits[p] = (hits[p].0, hits[p].1 + 1),
                        None => hits.push((target_type, 1)),
                    }
                    if pen_check(&weapon, &target_type){
                        let target_record = pens.iter().position(|x| std::ptr::eq(x.0, target_type)  );
                        match target_record {
                            Some(p) => pens[p] = (pens[p].0, pens[p].1 + 1),
                            None => pens.push((target_type, 1)),
                        }
                    }
                }
            }
        }
        println!("Force A Hits: '{:?}'", hits);
        println!("Force A Pens: '{:?}'", pens);
    }
    // FB
}

fn main() {
    let force_a = Force {
        name: String::from("Force A"),
        forces: vec![
            Element {unit_type: swordsman(), count: 50},
            Element {unit_type: spearman(), count: 50}
            ]
    };
    let force_b = Force {
        name: String::from("Force B"),
        forces: vec![Element {unit_type: spearman(), count: 50}, 
        Element {unit_type: swordsman(), count: 50}
        ]
    };
    run_round(force_a, force_b);
}
