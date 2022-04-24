use rfind::{Config, App};

fn main() {
    let config = Config::new();
    let app = App::new(config);

    app.run();
}
