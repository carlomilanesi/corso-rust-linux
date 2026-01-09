fn main() {
    for v in std::env::vars() {
        println!("{} = [{}]", v.0, v.1);
    }
}
