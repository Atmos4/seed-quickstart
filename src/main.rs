use seed::{prelude::*, *};

// Init
fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

// Model
struct Model {
    counter: i32,
}

// Update
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// View
fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

// Start
fn main() {
    App::start("app", init, update, view);
}