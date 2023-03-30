use commache::config;

fn main() {
    dbg!(config::get());
    let args: Vec<String> = std::env::args().collect();
    let arrgs: Vec<&str> = args.iter().map(|a| a.as_str()).collect();
    dbg!(&arrgs);

    commache::run(&arrgs);
}
