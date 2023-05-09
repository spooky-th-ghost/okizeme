use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn input_to_action_benchmark(c: &mut Criterion) {
    use bevy::utils::HashMap;
    use okizeme::character::actions::{Action, SingleHitbox};
    use okizeme::{ActionLibrary, ButtonMask, ButtonStream, CommandInput, InputTree, MotionMask};

    let b_stream = black_box(ButtonStream::with_buttons(
        ButtonMask::new(1),
        ButtonMask::new(1),
        ButtonMask::new(0),
    ));

    let input_tree = black_box(InputTree::from_input(
        "22333655222336",
        b_stream,
        MotionMask::new(1),
    ));

    let mut actions: HashMap<CommandInput, Box<dyn Action>> = black_box(HashMap::new());
    actions.insert(
        CommandInput::new(okizeme::CommandMotion::Qcf, "a"),
        Box::new(SingleHitbox::default()),
    );

    let action_library = black_box(ActionLibrary::new(actions));

    c.bench_function("find action", |b| {
        b.iter(|| action_library.find_action(&input_tree))
    });
}

criterion_group!(benches, input_to_action_benchmark);
criterion_main!(benches);
