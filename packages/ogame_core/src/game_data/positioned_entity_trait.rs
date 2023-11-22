pub trait PositionedEntity {
    fn get_real_position(&self) -> (i32, i32);
    fn distance_to<T: PositionedEntity>(&self, item: &T) -> i64 {
        let item1_real_coords = self.get_real_position();
        let item2_real_coords = item.get_real_position();

        ((item1_real_coords.0 as i64 - item2_real_coords.0 as i64).pow(2) as f64
            + (item1_real_coords.1 as i64 - item2_real_coords.1 as i64).pow(2) as f64)
            .sqrt() as i64
    }
}
