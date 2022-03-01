//! The frender version of
//! [ReactJs Tic Tac Toe](https://codepen.io/gaearon/pen/gWWZgR?editors=0010)

pub mod data;

use frender::prelude::*;

def_props! {
    pub struct SquareProps {
        pub value<TNode: react::Node>(v: TNode) -> react::AnyNode {
            v.into_node()
        },
        pub on_click: react::AnyFn<dyn Fn()>,
    }
}

#[component]
fn Square(props: &SquareProps) {
    rsx!(
        <button class="square" on_click={props.on_click.clone()}>
            {&props.value}
        </button>
    )
}

def_props! {
    pub struct BoardProps {
        pub board: data::Board,
        pub on_click: react::AnyFn<dyn Fn(usize)>,
    }
}

#[component]
fn Board(props: &BoardProps) {
    let render_square = |i: usize| {
        let on_click = props.on_click.clone();
        rsx!(
            <Square
                value:={props.board.squares[i].to_str()}
                on_click={move || on_click(i)}
            />
        )
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

#[component]
fn Game() {
    let (state, state_setter) = react::use_state!(() => data::Game::new());

    let current = state.current();
    let winner = current.calculate_winner();

    let status = match winner {
        data::Square::Empty => format!("Next player: {}", state.next_player().to_str()),
        _ => format!("Winner: {}", winner.to_str()),
    };

    let on_click = {
        let state_setter = state_setter.clone();
        move |i| {
            state_setter.set_optional_from_old(move |old| {
                let mut game = (**old).clone();
                if game.click(i) {
                    // game state changed
                    Some(game)
                } else {
                    // game state not changed
                    None
                }
            })
        }
    };

    let jump_to = move |i| {
        state_setter.set_optional_from_old(move |old| {
            let mut game = (**old).clone();
            if game.jump_to(i) {
                // game state changed
                Some(game)
            } else {
                // game state not changed
                None
            }
        })
    };

    let moves = (0..state.full_history().len())
        .map(|i: usize| {
            let jump_to = jump_to.clone();
            let desc = if i > 0 {
                format!("Go to move #{}", i)
            } else {
                "Go to game start".to_string()
            };
            rsx!(
              <li key={i}>
                <button on_click={move || jump_to(i)}>{desc}</button>
              </li>
            )
        })
        .collect::<Vec<_>>();

    rsx!(
        <div class="game">
            <div class="game-board">
                <Board
                    board={*current}
                    on_click={on_click}
                />
            </div>
            <div class="game-info">
                <div>{status}</div>
                <ol>{moves}</ol>
            </div>
        </div>
    )
}

#[component(main(mount_element_id = "frender-root"))]
fn Main() {
    rsx!(
        <div style={style! {
            "margin": "auto",
            "padding": 16,
            "maxWidth": 768,
        }}>
            <h1>
                "Tic Tac Toe - "
                <i>
                    <a href="https://github.com/frender-rs/frender" target={html::AnchorTarget::Blank}>
                        <b children="f" />
                        "render"
                    </a>
                </i>
            </h1>
            <p>
                "This is the frender version of "
                <a href="https://codepen.io/gaearon/pen/gWWZgR?editors=0010" target={html::AnchorTarget::Blank}>"ReactJs Tic Tac Toe"</a>
            </p>
            <main style={style!{ "marginTop": "32px" }}>
                <Game />
            </main>
        </div>
    )
}
