use crate::game::*;
use std::{
  cell::RefCell,
  rc::Rc,
};

pub fn clone_and_zip<'a, T, U>(
  iter: impl Iterator<Item = T> + 'a,
  cloneable: &'a U,
) -> impl Iterator<Item = (T, U)> + 'a
where
  U: Clone,
{
  iter.map(move |item| (item, cloneable.clone()))
}

// math: 26 + 2 + 26 + 2 + 26 = 82
// outer margin: 9
smithy_css::static_css!(
  .board {
    width: 82vmin;
    height: 82vmin;
    margin: 9vmin auto;
    background-color: #EEE;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    flex-wrap: wrap;

    box-shadow: 4px 4px 8px 0 rgba(7, 22, 30, 0.08);
  }

  .square {
    background-color: white;
    height: 26vmin;
    flex: 0 1 26vmin;
    margin-bottom: 2vmin;
    box-shadow: 4px 4px 8px 0 rgba(7, 22, 30, 0.08);

    text-align: center;
    font-size: 120px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    font-family: courier;
    color: #336;
  }

  .square_selectable {
    cursor: pointer;
  }

  .overlay {
  }

  .overlay_text {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    text-align: center;
    display: flex;
    flex-direction: column;
    justify-content: center;

    color: red;
    font-size: 90px;
    font-family: courier;
  }

  .overlay_background {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    opacity: 0.8;
    background-color: white;
  }
);

pub fn render<'a>(mut board: Board) -> smithy::types::SmithyComponent<'a> {
  let mut current_player = Rc::new(RefCell::new(Player::X));

  smithy::smd!(
    <style type="text/css">{ css.as_css_string() }</style>
    <div class={css.classes.board}>
      {
        clone_and_zip(board.iter_mut(), &current_player).map(|(mut item, current_player)| {
          if let Some(player) = item {
            smithy::smd!(<div
              class={css.classes.square}
            >
              { player.to_string() }
            </div>)
          } else {
            smithy::smd!(<div
              class={format!("{} {}", css.classes.square, css.classes.square_selectable)}
              on_click={|_| {
                *item = Some(*current_player.borrow());
                current_player.borrow_mut().next();
              }}
            />)
          }
        }).collect::<Vec<smithy::types::SmithyComponent>>()
      }
    </div>
    {
      if let Some(player) = board.winner() {
        Some(smithy::smd!(
          <div class={css.classes.overlay_background} />
          <div class={css.classes.overlay_text}>
            Player { player.to_string() }{' '}won!
          </div>
        ))
      } else {
        None
      }
    }
  )
}
