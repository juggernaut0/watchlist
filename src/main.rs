use azul::prelude::*;
use azul::widgets::button::Button;
use azul::widgets::label::Label;

struct IncrementPanel {
    counter: i32
}

impl Layout for IncrementPanel {
    fn layout(&self, layout_info: LayoutInfo<Self>) -> Dom<Self> {
        Dom::new(NodeType::Div)
            .with_child(Label::new(format!("{}", self.counter)).dom())
            .with_child(Button::with_label("Click me")
                .dom()
                .with_callback(On::MouseUp, Callback(|state, _| {
                    state.data.modify(|it| it.counter += 1)?;
                    Redraw
                })))
    }
}

fn main() {
    let mut app = App::new(IncrementPanel { counter: 0 }, AppConfig::default()).unwrap();
    let window = app
        .create_window(WindowCreateOptions::default(), css::native())
        .unwrap();
    app.run(window).unwrap();
}
