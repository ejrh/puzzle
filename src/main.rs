use bevy::app::App;

use puzzle::MainPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(MainPlugin);

    app.run();
}
