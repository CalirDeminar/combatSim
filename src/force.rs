pub mod element;
use element::Element;


#[derive(Debug, Clone)]
pub struct Force {
    pub name: String,
    pub forces: Vec<Element>,
}



pub mod force_combat {
    mod logger;
    use crate::units;
    use crate::Force;
    use crate::Element;
    use logger::{*};
    use units::{Unit, weapons};
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
    
    fn kill_check(weapon: &Weapon, target: &Unit) -> bool {
        let mut rng = rand::thread_rng();
        let i: f32 = rng.gen();
        let pen_chance = weapon.pen as f32/(target.hp * 2) as f32;
        return i < pen_chance.clamp(0.0, 1.0);
    }

    fn random_target(force: &mut Force) -> Option<&mut Element>  {
        let elements = &mut force.forces;
        let total_forces = elements.into_iter().fold(0, |acc, e| acc + e.count);
        let odds = elements.into_iter().map(|e| (e.count as f32 / total_forces as f32, e));
        let mut i: f32 = rand::thread_rng().gen();
        for entry in odds {
            if i < entry.0 {
                return Some(entry.1);
            }
            i -= entry.0;
        }
        return None;
    }

    pub fn calculate_losses(attacker: Force, target: Force) -> (Force, Vec<CombatLog>) {
        let mut working_target = target.clone();
        let mut combat_log: Vec<CombatLog> = vec![];
        for element in attacker.forces {
            for _i in 1..(element.count + 1) {
                for weapon in &element.unit_type.weapons {
                    let target_option = random_target(&mut working_target);
                    if target_option.is_some() {
                        let target_element = target_option.unwrap();
                        let hit = hit_check(weapon, &target_element.unit_type);
                        let penned = hit && pen_check(weapon, &target_element.unit_type);
                        let killed = penned && kill_check(weapon, &target_element.unit_type);
                        let mut verb = String::from("missed");
                        if hit {
                            verb = String::from("hit");
                        }
                        if penned {
                            verb = String::from("penned");
                        }
                        if killed {
                            verb = String::from("killed");
                            target_element.count -= 1;
                        }
                        combat_log.push(
                            CombatLog {
                                attacker_force_name: attacker.name.clone(),
                                attacker_unit_name: element.unit_type.name.clone(),
                                weapon_name: weapon.name.clone(),
                                target_force_name: target.name.clone(),
                                target_unit_name: target_element.unit_type.name.clone(),
                                verb: verb,
                            }
                        );
                    }
                }
            }
        }
        return (working_target, combat_log);
    }

    pub fn run_round(force_a: Force, force_b: Force) -> (Force, Force, Vec<CombatLog>) {
        let mut combat_log = vec![];
        let (force_b_rtn, mut log_a) = calculate_losses(force_a.clone(), force_b.clone());
        let (force_a_rtn, mut log_b) = calculate_losses(force_b.clone(), force_a.clone());
        combat_log.append(&mut log_a);
        combat_log.append(&mut log_b);
        return (force_a_rtn, force_b_rtn, combat_log);
    }

    pub fn run_combat(force_a: Force, force_b: Force) {
        let mut f_a = force_a.clone();
        let mut f_b = force_b.clone();
        let mut round_count = 0;
        loop {
            println!("\nRound: {:?}", round_count);
            print_combatants(&f_a);
            print_combatants(&f_b);
            let res = run_round(f_a.clone(), f_b.clone());
            // println!("Log: {:#?}", (res.0.clone(), res.1.clone()));
            f_a = res.0;
            f_b = res.1;
            f_a.forces.retain(|e| e.count > 0);
            f_b.forces.retain(|e| e.count > 0);
            print_combat_log(&res.2, f_a.name.clone());
            print_combat_log(&res.2, f_b.name.clone());
            if f_a.forces.len() == 0 {
                println!("{:?} is victorious", force_b.name);
                break;
            }
            if f_b.forces.len() == 0 {
                println!("{:?} is victorious", force_a.name);
                break;
            }
            round_count += 1;
        }
    }
}