pub struct Hurtbox {
  pub is_grounded: bool,
  pub block_state: BlockState,
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockState {
  Stand {barrier: bool, instant: bool},
  Air {barrier: bool, instant: bool},
  Crouch {barrier: bool, instant: bool},
  None
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockModifier {
  Barrier,
  Instant,
}
