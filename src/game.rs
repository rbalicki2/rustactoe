#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Player {
  X,
  O,
}

impl std::string::ToString for Player {
  fn to_string(&self) -> String {
    match self {
      Player::X => "X",
      Player::O => "O",
    }
    .to_string()
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

  pub fn winner(&self) -> Option<Player> {
    let inner = self.0;
    for offset in 0..3 {
      // columns
      if inner[offset].is_some()
        && inner[offset] == inner[offset + 3]
        && inner[offset] == inner[offset + 6]
      {
        return inner[offset];
      }
      // rows
      if inner[offset * 3].is_some()
        && inner[offset * 3] == inner[offset * 3 + 1]
        && inner[offset * 3] == inner[offset * 3 + 2]
      {
        return inner[offset * 3];
      }
    }

    // diagonals
    if inner[0].is_some() && inner[0] == inner[4] && inner[0] == inner[8] {
      return inner[0];
    }

    if inner[2].is_some() && inner[2] == inner[4] && inner[2] == inner[6] {
      return inner[2];
    }

    None
  }

  pub fn next_player(&self) -> Player {
    Player::X
  }
}
