//! The frender version of
//! [ReactJs Tic Tac Toe](https://codepen.io/gaearon/pen/gWWZgR?editors=0010)

pub mod data;

use frender::prelude::*;

bg::builder! {
    pub struct SquareProps {
        value[impl frender::UpdateRenderState<frender::Dom> + 'static],
        on_click[impl frender::UpdateDomEventListener<frender::events::Click>],
    }
}

#[component(only_dom)]
fn Square(ctx: _, props: SquareProps) {
    ctx.render(rsx!(
        <button class="square" on_click={props.on_click}>
            {props.value}
        </button>
    ))
}

bg::builder! {
    pub struct BoardProps {
        board: data::Board,
        on_click[impl Fn(usize) + 'static + Clone],
    }
}

#[component(only_dom)]
fn Board(ctx: _, props: BoardProps) {
    let render_square = |i: usize| {
        let on_click = props.on_click.clone();
        rsx!(
            <Square
                value={props.board.squares[i].to_str()}
                on_click={move |_: &_| on_click(i)}
            />
        )
    };

    ctx.render(rsx!(
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
    ))
}

#[component(only_dom)]
fn Game(ctx: _) {
    let (state, state_setter) = hooks::use_state_with(data::Game::new);

    let current = state.current();
    let state_setter = state_setter.clone();
    let winner = current.calculate_winner();

    let status = match winner {
        data::Square::Empty => format!("Next player: {}", state.next_player().to_str()),
        _ => format!("Winner: {}", winner.to_str()),
    };

    let on_click = {
        let state_setter = state_setter.clone();
        move |i| {
            state_setter.mutate_with_fn_box(move |game| {
                game.click(i);
            })
        }
    };

    let jump_to = move |i| {
        state_setter.mutate_with_fn_box(move |game| {
            game.jump_to(i);
        })
    };

    let moves = (0..state.full_history().len())
        .map(|i: usize| {
            let jump_to = jump_to.clone();
            let desc = if i > 0 {
                format!("Go to move #{i}")
            } else {
                "Go to game start".to_string()
            };

            rsx!(
              <li key={i}>
                <button on_click={move |_: &_| jump_to(i)}>{desc}</button>
              </li>
            )
        })
        .collect::<Vec<_>>();

    ctx.render(rsx!(
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
    ))
}

#[component(only_dom)]
fn Main(ctx: _) {
    ctx.render(rsx!(
        <div style=r#"
            margin: auto;
            padding: 16px;
            max-width: 768px;
        "#>
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
            <main style="margin-top: 32px">
                <Game />
            </main>
        </div>
    ))
}

fn main() {
    ::frender::hook_element::frender_dom::spawn_mount_get_element_to_dom_element_by_id(
        || rsx!(Main()),
        "frender-root",
    );
}
