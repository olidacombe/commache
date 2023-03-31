use std::process::Command;

pub fn run(cmd: &str, args: &[&str]) {
    println!("{:?} {:?}", cmd, args);
    let output = Command::new(cmd).args(args).output();
    //
    if let Ok(output) = output {
        dbg!(output);
    }
}
