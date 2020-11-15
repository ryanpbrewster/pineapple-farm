// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use std::collections::HashMap;


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
    counters: HashMap<(i32, i32), i32>,
}
impl Model {
    fn new(width: i32, height: i32) -> Model {
        let mut counters = HashMap::new();
        for i in 0 .. height {
            for j in 0 .. width {
                counters.insert((i, j), 0);
            }
        }
        Model { counters } 
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
        Msg::Increment(a, b) => {
            if let Some(v) = model.counters.get_mut(&(a, b)) {
                *v += 1;
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let buttons: Vec<Node<Msg>> = model.counters.iter().map(|(&(i, j), &v)| {
        button![v, ev(Ev::Click, move |_| Msg::Increment(i, j))]
    }).collect();
    div![
        "This is a counter: ",
        C!["counter"],
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
