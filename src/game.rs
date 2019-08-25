#[derive(Debug, Clone)]
pub enum Player {
  X,
  O,
}

impl std::string::ToString for Player {
  fn to_string(&self) -> String {
    match self {
      Player::X => "X",
      Player::O => "O",
    }.to_string()
  }
}

impl Player {
  pub fn next(&mut self) {
    match self {
      Player::X => *self = Player::O,
      Player::O => *self = Player::X,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Board(pub [Option<Player>; 9]);

impl std::ops::Deref for Board {
  type Target = [Option<Player>; 9];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for Board {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Board {
  pub fn empty() -> Board {
    Board([None, None, None, None, None, None, None, None, None])
  }

  pub fn perform_best_move(&mut self, player: Player) {
    // for some meaning of "best"
    for item in self.iter_mut() {
      match item {
        None => {
          *item = Some(player);
          return;
        },
        Some(_) => {},
      }
    }
  }

  pub fn next_player(&self) -> Player {
    Player::X
  }
}
