fn main() {
    let stdin = std::io::stdin();
    let json_object = serde_json::from_reader(stdin).expect("error while reading");
    let schema = infers_jsonschema::infer(&json_object);
    let pretty = serde_json::to_string_pretty(&schema).expect("couldn't prettify schema");

    println!("{}", pretty);
}
