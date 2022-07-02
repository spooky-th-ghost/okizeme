use crate::{
    Hitbox,
    StunValue,
    ComboedState
};
/// Resource used to track player combos, as well as calculate damage and hitstun for those combos
pub struct Combo {
    hit_count: u8,
    valid: bool,
    hitstun_modifier: u8,
    damage_scaling: f32,
    total_damage: u16,
    breakpoints: Vec<u8>
}

impl Combo {
    /// Create a new combo from a connected hitbox
    pub fn new(hitbox: Hitbox) -> Self {
        Combo {
            hit_count: 1,
            valid: true,
            hitstun_modifier: 0,
            damage_scaling: hitbox.proration(),
            total_damage: hitbox.damage(),
            breakpoints: Vec::new()
        }
    }

    /// Add a hit to a combo and return the damage and hitstun values to apply
    pub fn add_to_combo(&mut self, hitbox: Hitbox, missed_tech: bool, comboed_state: ComboedState) -> (u16,u8) {
        let stun_value = StunValue::from_attack_level(hitbox.level());
        self.add_hit(missed_tech);
        let adjusted_damage = self.scaled_damage(hitbox.damage());
        self.total_damage += adjusted_damage;
        let hit_stun = self.calculate_hitstun(comboed_state, stun_value);
        (adjusted_damage, hit_stun)
    }

    fn add_hit(&mut self, missed_tech: bool) {
        if missed_tech {
            self.valid = false;
            self.breakpoints.push(self.hit_count);
            if self.damage_scaling < 0.7 {
                self.damage_scaling = 0.7;
            }

            if self.hitstun_modifier > 5 {
                self.hitstun_modifier = 5;
            }
        } else {
            self.damage_scaling -= 0.3;
            self.hitstun_modifier += 3;
        }
        self.damage_scaling = self.damage_scaling.clamp(0.3,1.5);
        self.hitstun_modifier = self.hitstun_modifier.clamp(0, 12);
        self.hit_count += 1;
    }

    fn calculate_hitstun(&self,comboed_state: ComboedState, stun_value: StunValue) -> u8 {
        use ComboedState::*;
        match comboed_state {
            Standing => stun_value.standing_hitstun - self.hitstun_modifier,
            Crouching => stun_value.crouching_hitstun - self.hitstun_modifier,
            Juggle => stun_value.aerial_hitstun - self.hitstun_modifier
        }
    }

    fn scaled_damage(&self, base_damage: u16) -> u16 {
        (base_damage as f32 * self.damage_scaling) as u16
    }
}
