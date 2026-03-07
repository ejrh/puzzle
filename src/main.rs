use bevy::app::App;

use puzzle::PuzzlePlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(PuzzlePlugin);

    app.run();
}
