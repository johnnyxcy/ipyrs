#[derive(serde::Deserialize)]
struct Data {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let d = serde_json::from_str::<Data>(data);

    match d {
        Ok(d) => {
            println!("name = {}", d.name);
            println!("age = {}", d.age);
            println!("phones = {:?}", d.phones);
        }
        _ => println!("error"),
    };

    println!("Hello, world!");
}
