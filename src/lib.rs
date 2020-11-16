// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::models::{Cell, Grid, GrowthStatus};
mod models;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::new(5, 5)
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counters: Grid,
}
impl Model {
    fn new(width: i32, height: i32) -> Model {
        Model {
            counters: Grid::new(width, height),
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
                *radiators = 1 - *radiators;
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
    let buttons: Vec<Node<Msg>> = model
        .counters
        .iter()
        .map(|((i, j), cell)| {
            let status = model.counters.get_status(&(i, j)).unwrap();
            let content = format!(
                "{} [r={},c={}] ({}/{})",
                model.counters.get_heat(&(i, j)),
                cell.radiators,
                cell.capacity,
                i,
                j
            );
            button![
                content,
                ev(Ev::Click, move |_| Msg::Increment(i, j)),
                style! {
                    St::GridArea => format!("{}/{}", i + 1, j + 1),
                    St::Background => match status {
                        GrowthStatus::TooCold => "blue",
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
            St::Grid => "repeat(5, 1fr) / repeat(5, 1fr)",
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
