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
        let mut element_hits = 0;
        for _i in 1..element.count {
            for weapon in element.unit_type.weapons {
                let target_type = random_target(&force_b);
                if hit_check(&weapon, &target_type) {
                    element_hits += 1;
                }
            }
        }
        println!("Force A Hits: '{}'", element_hits);
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
