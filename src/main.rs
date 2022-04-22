use rfind::{App, Config};

use std::env;

fn main() {
    let args = env::args();

    let config = Config::new(args);
    let app = App::new(config);

    app.run();
}
