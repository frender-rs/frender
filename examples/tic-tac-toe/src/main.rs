//! The frender version of
//! [ReactJs Tic Tac Toe](https://codepen.io/gaearon/pen/gWWZgR?editors=0010)

pub mod data;

use frender::prelude::*;

pub struct Square<V, OnClick>
where
    V: Element,
    OnClick: MaybeHandleEvent<dyn frender::MouseEvent> + 'static,
{
    value: V,
    on_click: OnClick,
}

impl<V, OnClick> Square<V, OnClick>
where
    V: Element,
    OnClick: MaybeHandleEvent<dyn frender::MouseEvent> + 'static,
{
    // #[component(only_dom)] // TODO: optimize with zero hooks
    fn into_element(self) -> impl Element {
        rsx!(
            <button class="square" on_click={self.on_click}>
                {self.value}
            </button>
        )
    }
}

pub struct Board<OnClick: Fn(usize) + Clone + 'static> {
    board: data::Board,
    on_click: OnClick,
}

impl<OnClick: Fn(usize) + Clone + 'static> Board<OnClick> {
    #[component]
    fn into_element(self) {
        let render_square = |i: usize| {
            let on_click = self.on_click.clone();
            Square {
                value: self.board.squares[i].to_str(),
                on_click: move |_: &_| on_click(i),
            }
            .into_element()
        };

        rsx!(
            <div>
                <div class="board-row">
                    {render_square(0)}
                    {render_square(1)}
                    {render_square(2)}
                </div>
                <div class="board-row">
                    {render_square(3)}
                    {render_square(4)}
                    {render_square(5)}
                </div>
                <div class="board-row">
                    {render_square(6)}
                    {render_square(7)}
                    {render_square(8)}
                </div>
            </div>
        )
    }
}

#[component]
fn Game() {
    let (state, state_setter) =
        hooks::use_shared_reduce_with(|| (data::Game::new(), data::Game::reduce));

    let current = state.current();
    let winner = current.calculate_winner();

    let status = match winner {
        data::Square::Empty => format!("Next player: {}", state.next_player().to_str()),
        _ => format!("Winner: {}", winner.to_str()),
    };

    let on_click = {
        let state_setter = state_setter.clone();
        move |i| state_setter.dispatch(data::GameAction::Click(i))
    };

    let jump_to = {
        let state_setter = state_setter.clone();
        move |i| state_setter.dispatch(data::GameAction::JumpTo(i))
    };

    let moves = KeyedElements((0..state.full_history().len()).map(move |i: usize| {
        let desc = if i > 0 {
            format!("Go to move #{i}")
        } else {
            "Go to game start".to_string()
        };

        let jump_to = jump_to.clone();
        rsx!(
          <li key={i}>
            <button on_click={move |_: &_| jump_to(i)}>{desc}</button>
          </li>
        )
    }));

    rsx!(
        <div class="game">
            <div class="game-board">
                {
                    Board {
                        board: *current,
                        on_click,
                    }.into_element()
                }
            </div>
            <div class="game-info">
                <div>{status}</div>
                <ol>{moves}</ol>
            </div>
        </div>
    )
}

#[component(main(get_dom_element = "frender-root"))]
fn Main() {
    rsx!(
        <div style={style!(r#"
            margin: auto;
            padding: 16px;
            max-width: 768px;
        "#)}>
            <h1>
                "Tic Tac Toe - "
                <i>
                    <a href="https://github.com/frender-rs/frender" target="_blank">
                        <b children="f" />
                        "render"
                    </a>
                </i>
            </h1>
            <p>
                "This is the frender version of "
                <a href="https://codepen.io/gaearon/pen/gWWZgR?editors=0010" target="_blank">"ReactJs Tic Tac Toe"</a>
            </p>
            <main style={style!("margin-top: 32px")}>
                {Game()}
            </main>
        </div>
    )
}
