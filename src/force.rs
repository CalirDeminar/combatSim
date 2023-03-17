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
    use crate::{Force, Unit, Element};
    use logger::{*};
    use simulation::simulation::{*};

    pub fn element_builder(unit_type: Unit, count: i32, fort: i32) -> Element {
        return Element {unit_type: unit_type, count: count, fortification: fort};
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