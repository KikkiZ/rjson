use rjson::Parser;

#[allow(dead_code)]
#[derive(Debug, Parser)]
pub struct Command {
    executable: String,
    args: Vec<String>,
}

fn main() {
    let command = Command::parser()
        .executable("find")
        .args(vec!["-c".into(), "-v".into()])
        .finish()
        .unwrap();

    println!("{:?}", command);
}
