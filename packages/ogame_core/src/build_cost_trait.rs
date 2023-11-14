use crate::resources::Resources;

pub trait BuildCost {
    fn cost(&self, level: usize) -> Resources;
}
