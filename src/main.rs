fn main() {
    let stdin = std::io::stdin();
    let v = serde_json::from_reader(stdin).expect("error while reading");

    let schema = infers_jsonschema::infer(&v);
    println!("{}", schema);
}
