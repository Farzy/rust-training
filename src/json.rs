#[derive(Deserialize, Debug, Clone)]
struct Person {
    name: String,
}

impl Default for Person {
    fn default() -> Self {
        Person { name: "Unknown person implemented as default".to_string() }
    }
}

pub fn main() {
    // Broken JSON
    let broken_json = serde_json::from_str::<Person>(r#"{
            "name": "Margaret Hamilton",
        }"#);
    // First version with match() on Result<>
    let first_inner = match &broken_json {
        Ok(inner) => inner.clone(),
        Err(e) => Person { name: String::from(format!("unknown ({})", *e)) },
    };
    println!("First name: {:?}", first_inner.name);
    // Next version is optimized
    let first_inner = broken_json
        .unwrap_or(Person { name: String::from("Unknown person") });
    println!("First name: {:?}", first_inner.name);
    // Third version uses default value
    // NOTE: unwrap_or() took ownership of the content of Result≤>, adding "&" did not
    //       solve the problem. So let's create the JSON again.
    let broken_json = serde_json::from_str::<Person>(r#"{
            "name": "Margaret Hamilton",
        }"#);
    let first_inner = broken_json.unwrap_or_default();
    println!("First name: {:?}", first_inner.name);

    // Valid JSON
    let first = serde_json::from_str::<Person>(r#"{
            "name": "Margaret Hamilton"
        }"#);
    // First version with match() on Result<>
    let first_inner = match &first {
        Ok(inner) => inner.clone(),
        Err(e) => Person { name: String::from(format!("unknown ({})", *e)) },
    };
    println!("First name: {:?}", first_inner.name);
    // Next version is optimized
    let first_inner = first.unwrap_or(Person { name: String::from("Unknown person") });
    println!("First name: {:?}", first_inner.name);
}
