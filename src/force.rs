pub mod element;
use element::Element;

#[derive(Debug)]
pub struct Force {
    pub name: String,
    pub  forces: Vec<Element>,
}