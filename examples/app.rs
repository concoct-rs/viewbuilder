use concoct::{Context, Slot, Object};
use viewbuilder::{
    view::{LinearLayout, Text},
    window, UserInterface, Window,
};

struct App;

impl Object for App {}

impl Slot<window::Resized> for App {
    fn handle(&mut self, _handle: Context<Self>, msg: window::Resized) {
        dbg!(msg);
    }
}

fn main() {
    let ui = UserInterface::default();
    let _guard = ui.enter();

    let app = App.spawn();

    let window = Window::new(LinearLayout::new((Text {}, Text {}))).spawn();
    window.bind(&app);

    ui.run()
}
