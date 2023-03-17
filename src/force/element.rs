pub mod units;
use units::Unit;

#[derive(Debug, Clone)]
pub struct Element {
    pub unit_type: Unit,
    pub count: i32,
    // 0-10
    pub fortification: i32
}