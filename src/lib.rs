// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use rand::Rng;
use seed::{prelude::*, *};

use crate::models::{Cell, Grid, GrowthStatus};
mod models;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let mut prng = rand::thread_rng();
    Model::random(&mut prng)
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counters: Grid,
    radiators_available: u32,
}
impl Model {
    fn random<R: Rng>(prng: &mut R) -> Model {
        let width = prng.gen_range(4, 10);
        let height = prng.gen_range(4, 10);
        Model {
            counters: Grid::random(width, height, prng),
            radiators_available: 5,
        }
    }
}

// ------ ------
//    Update
// ------ ------
// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment(i32, i32),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment(a, b) => match model.counters.get_mut(&(a, b)) {
            None => {}
            Some(Cell { radiators, .. }) => {
                if *radiators == 1 {
                    *radiators = 0;
                    model.radiators_available += 1;
                } else if model.radiators_available > 0 {
                    *radiators = 1;
                    model.radiators_available -= 1;
                }
            }
        },
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        view_grid(&model.counters),
        div![format!("{} radiators available", model.radiators_available),],
        div![format!("{} pineapples", model.counters.total_growth()),]
    ]
}
fn view_grid(grid: &Grid) -> Node<Msg> {
    let (width, height) = grid.dimensions();
    let buttons: Vec<Node<Msg>> = grid
        .iter()
        .map(|((i, j), cell)| {
            let status = grid.get_status(&(i, j)).unwrap();
            button![
                cell.capacity,
                ev(Ev::Click, move |_| Msg::Increment(i, j)),
                style! {
                    St::GridArea => format!("{}/{}", i + 1, j + 1),
                    St::Background => match status {
                        GrowthStatus::TooCold => "lightblue",
                        GrowthStatus::TooHot => "orange",
                        GrowthStatus::Overheated => "red",
                        GrowthStatus::Fruiting(_) => "green",
                    },
                }
            ]
        })
        .collect();
    div![
        style! {
            St::Display => "grid",
            St::Grid => format!("repeat({}, 1fr) / repeat({}, 1fr)", height, width),
            St::Width => "640px",
            St::Height => "640px",
        },
        buttons,
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
