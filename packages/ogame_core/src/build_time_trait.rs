use crate::build_cost_trait::BuildCost;

pub trait BuildTime: BuildCost {
    fn build_time(&self, level: usize) -> usize {
        self.cost(level).build_time()
    }
}

impl<T: BuildCost> BuildTime for T {}
