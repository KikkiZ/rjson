use rjson::Parse;

#[allow(dead_code)]
#[derive(Debug, Parse)]
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
