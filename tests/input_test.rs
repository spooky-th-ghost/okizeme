#[cfg(test)]
mod test {
    #[test]
    fn test_action_library() {
        use bevy::utils::HashMap;
        use okizeme::character::actions::{Action, SingleHitbox};
        use okizeme::{
            ActionLibrary, ButtonMask, ButtonStream, CommandInput, InputTree, MotionMask,
        };

        let b_stream =
            ButtonStream::with_buttons(ButtonMask::new(1), ButtonMask::new(1), ButtonMask::new(0));

        let input_tree = InputTree::from_input("22333655222336", b_stream, MotionMask::new(1));

        let mut actions: HashMap<CommandInput, Box<dyn Action>> = HashMap::new();
        actions.insert(
            CommandInput::new(okizeme::CommandMotion::Qcf, "a"),
            Box::new(SingleHitbox::default()),
        );

        let action_library = ActionLibrary::new(actions);

        assert!(action_library.find_action(&input_tree).is_some());
    }
}
