pub enum Player {
  X,
  O,
}

pub struct Board([[Option<Player>; 3]; 3]);

impl Board {
  pub fn empty() -> Board {
    Board([[None, None, None], [None, None, None], [None, None, None]])
  }

  pub fn render(&mut self) -> smithy::types::SmithyComponent {
    smithy::smd!(board)
  }
}
