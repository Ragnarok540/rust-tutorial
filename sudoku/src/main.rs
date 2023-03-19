use piston::WindowSettings;
use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings, Events};

fn main() {
    let settings = WindowSettings::new("Sudoku", (640, 480)).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());

    //while let some

    println!("{}", settings.get_exit_on_esc());
}
