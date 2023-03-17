pub mod element;
use element::Element;


#[derive(Debug, Clone)]
pub struct Force {
    pub name: String,
    pub forces: Vec<Element>,
}



pub mod force_combat {
    mod logger;
    mod simulation;
    use crate::Force;
    use logger::{*};
    use simulation::simulation::{*};

    pub fn calculate_losses(attacker: Force, target: Force) -> (Force, Vec<CombatLog>) {
        let mut working_target = target.clone();
        let mut combat_log: Vec<CombatLog> = vec![];
        for element in attacker.forces {
            for _i in 1..(element.count + 1) {
                for weapon in &element.unit_type.weapons {
                    for _j in 1..(weapon.rof + 1) {
                        let target_option = random_target(&mut working_target);
                        if target_option.is_some() {
                            let target_element = target_option.unwrap();
                            let hit = hit_check(weapon, &target_element.unit_type);
                            let penned = hit && pen_check(weapon, &target_element.unit_type);
                            let killed = penned && kill_check(weapon, &target_element.unit_type);
                            let mut verb = MISSED;
                            if hit {
                                verb = HIT;
                            }
                            if penned {
                                verb = PENNED;
                            }
                            if killed {
                                verb = KILLED;
                                target_element.count -= 1;
                            }
                            combat_log.push(
                                CombatLog {
                                    attacker_force_name: attacker.name.clone(),
                                    attacker_unit_name: element.unit_type.name.clone(),
                                    weapon_name: weapon.name.clone(),
                                    target_force_name: target.name.clone(),
                                    target_unit_name: target_element.unit_type.name.clone(),
                                    verb: String::from(verb),
                                }
                            );
                        }
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
        let mut round_count = 1;
        loop {
            println!("\nRound: {:?}", round_count);
            print_combatants(&f_a);
            print_combatants(&f_b);
            let res = run_round(f_a.clone(), f_b.clone());
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