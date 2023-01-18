#[cfg(test)]
mod test {
    use bevy::prelude::*;
    use okizeme::{systems::manage_busy, types::Busy};

    #[test]
    fn is_busy_decremented() {
        // Setup world
        let mut world = World::default();

        // Setup stage with our two systems
        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(manage_busy);

        // Setup test entities
        let busy_id = world.spawn(Busy(1)).id();

        // Run systems
        update_stage.run(&mut world);

        // Check resulting changes
        assert!(world.get::<Busy>(busy_id).is_some());
    }

    #[test]
    fn is_busy_removed() {
        // Setup world
        let mut world = World::default();

        // Setup stage with our two systems
        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(manage_busy);

        // Setup test entities
        let busy_id = world.spawn(Busy(1)).id();

        // Run systems
        update_stage.run(&mut world);
        update_stage.run(&mut world);

        assert!(world.get::<Busy>(busy_id).is_none());
    }
}
