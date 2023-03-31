use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub cmd: Vec<String>,
}

impl Args {
    pub fn get(&self) -> Vec<&str> {
        self.cmd.iter().map(|a| a.as_str()).collect()
    }
}
