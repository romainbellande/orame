use super::GameData;

pub trait PositionedEntity {
    fn get_real_position(&self, game_data: &GameData) -> (i32, i32);
    fn distance_to<T: PositionedEntity>(&self, item: &T, game_data: &GameData) -> i64 {
        let item1_real_coords = self.get_real_position(game_data);
        let item2_real_coords = item.get_real_position(game_data);

        ((item1_real_coords.0 as i64 - item2_real_coords.0 as i64).pow(2) as f64
            + (item1_real_coords.1 as i64 - item2_real_coords.1 as i64).pow(2) as f64)
            .sqrt() as i64
    }
}
