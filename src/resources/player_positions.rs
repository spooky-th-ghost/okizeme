use crate::PlayerId;
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerPositions(Vec<Position>);

pub struct Position {
    pub player_id: PlayerId,
    position: Vec3,
}

impl Position {
    pub fn new(player_id: PlayerId, position: Vec3) -> Self {
        Position {
            player_id,
            position,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }
}

impl Default for PlayerPositions {
    fn default() -> Self {
        PlayerPositions(vec![
            Position::new(PlayerId::P1, Vec3::new(-50.0, 0.0, 0.0)),
            Position::new(PlayerId::P2, Vec3::new(50.0, 0.0, 0.0)),
        ])
    }
}

impl PlayerPositions {
    pub fn get_facing_vector(&self, player_id: &PlayerId) -> f32 {
        let p1_x_pos = self.0[0].get_position().x;
        let p2_x_pos = self.0[1].get_position().x;

        if p1_x_pos > p2_x_pos {
            if *player_id == PlayerId::P1 {
                -1.0
            } else {
                1.0
            }
        } else if *player_id == PlayerId::P1 {
            1.0
        } else {
            -1.0
        }
    }

    pub fn get_facing_right(&self, player_id: &PlayerId) -> bool {
        self.get_facing_vector(player_id) > 0.0
    }

    pub fn set_position(&mut self, player_id: &PlayerId, position: Vec3) {
        self.0
            .iter_mut()
            .find(|x| x.player_id == *player_id)
            .unwrap()
            .set_position(position);
    }

    pub fn get_position(&mut self, player_id: &PlayerId) -> Vec3 {
        self.0
            .iter()
            .find(|&x| x.player_id == *player_id)
            .unwrap()
            .get_position()
    }

    pub fn get_distance(&self) -> f32 {
        self.0[0].get_position().distance(self.0[1].get_position())
    }

    pub fn get_mid_point(&self) -> Vec2 {
        let p1 = self.0[0].get_position();
        let p2 = self.0[1].get_position();
        Vec2::new(p1.x + p2.x / 2.0, p1.y + p2.y / 2.0)
    }
}
