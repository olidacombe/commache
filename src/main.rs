use commache::config;

fn main() {
    dbg!(config::get());
    let args = std::env::args();

    commache::run(&["oi", "you"]);
}
