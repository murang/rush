mod nice;

use rush::{app::App};


fn main() {
    let mut app = App::new();
    app.add_component(nice::Nice::new());
    app.on_exit(||{
       println!("lets exit"); 
    });
    app.start_run(1);
}
