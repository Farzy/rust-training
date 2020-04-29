#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

pub fn main() {
    let broken_json = serde_json::from_str::<Person>(r#"{
            "name": "Margaret Hamilton",
        }"#);
    let first_inner = match broken_json {
        Ok(inner) => inner,
        Err(e) => Person { name: String::from(format!("unknown ({})", e)) },
    };
    println!("First name: {:?}", first_inner.name);

    let first = serde_json::from_str::<Person>(r#"{
            "name": "Margaret Hamilton"
        }"#);
    let first_inner = match first {
        Ok(inner) => inner,
        Err(e) => Person { name: String::from(format!("unknown ({})", e)) },
    };
    println!("First name: {:?}", first_inner.name);
}
