pub mod units;
use units::Unit;

#[derive(Debug)]
pub struct Element {
    pub unit_type: Unit,
    pub count: i32
}