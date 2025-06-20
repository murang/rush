use rush::app::App;

fn main() {
    let mut app = App::new();
    let mut counter = 0;
    app.on_exit(||{
       println!("lets exit"); 
    });
    app.run(10, move || {
        println!("tick {}",  counter.to_string());
        counter += 1;
    });
}
