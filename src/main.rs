use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

struct Model {
    counter: i32,
}

enum Msg {
    Increment,
    Decrement,
    Reset,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
        Msg::Reset => model.counter = 0,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        style! {
                St::Background => "#aaa",
                St::TextAlign => "center",
                St::Padding => "20px 0",
        },
        C!["counter"],
        h1![
            style! {
                        St::FontSize => "50px",
            },
            "This is a counter"
        ],
        p![
            style! {
                        St::FontSize => "50px",
            },
            model.counter
        ],
        button![
            style! {
                        St::FontSize => "40px",
                        St::Padding => "10px 20px",
                        St::Margin => "20px",
            },
            "+",
            ev(Ev::Click, |_| Msg::Increment),
        ],
        button![
            style! {
                        St::FontSize => "40px",
                        St::Padding => "10px 20px",
                        St::Margin => "20px",
            },
            "-",
            ev(Ev::Click, |_| Msg::Decrement),
        ],
        div![
            style! {
                        St::TextAlign => "center",
            },
            button![
                style! {
                            St::FontSize => "40px",
                            St::Padding => "10px 20px",
                            St::Margin => "20px",
                },
                "Reset",
                ev(Ev::Click, |_| Msg::Reset),
            ]
        ]
    ]
}

pub fn main() {
    App::start("app", init, update, view);
}
