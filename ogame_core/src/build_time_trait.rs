use crate::build_cost_trait::BuildCost;

pub trait BuildTime: BuildCost {
    fn build_time(&self, level: usize) -> usize {
        let cost = self.cost(level);

        let build_time =
            (cost.metal + cost.crystal) / (2500.0 * crate::UNIVERSE_SPEED as f64) * 3600.0;

        build_time.ceil() as usize
    }
}

impl<T: BuildCost> BuildTime for T {}
