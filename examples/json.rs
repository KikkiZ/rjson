use rjson::parser;

#[parser(path = "./template/Object.json")]
pub struct Object;

fn main() {
    let obj = Object {
        age: 10,
        name: "test".to_string(),
        phones: vec!["123".to_string()],
    };
    
    println!("{:#?}", obj);
}
