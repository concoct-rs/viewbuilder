use skia_safe::Color4f;
use viewbuilder::{Text, View};

#[tokio::main]
async fn main() {
    viewbuilder::transaction(|ui| {
        ui.insert(Text::builder().content("Hello World!").build());
    });

    viewbuilder::run();
}
