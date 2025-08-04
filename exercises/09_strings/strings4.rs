// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue"); // &str literal
    string(String::from("hi")); // String::from() creates String
    string("red".to_string()); // .to_string() creates String
    string("rust is fun!".to_owned()); // .to_owned() creates String
    string("nice weather".into()); // .into() creates String
    string(format!("Interpolation {}", "Station")); // format!() creates String
    string_slice(&String::from("abc")[0..1]); // slice of String is &str
    string_slice("  hello there ".trim()); // .trim() returns &str
    string("Happy Monday!".replace("Mon", "Tues")); // .replace() creates String
    string(format!("Interpolation {}", "Station")); // format!() creates String

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]); // slice of String is &str
    string_slice("  hello there ".trim()); // .trim() returns &str
    string("Happy Monday!".replace("Mon", "Tues")); // .replace() creates String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // .to_lowercase() creates String
}
