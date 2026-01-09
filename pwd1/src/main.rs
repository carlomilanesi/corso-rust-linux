fn main() {
    //println!("{}", std::env::current_dir().unwrap().to_string_lossy());
    match std::env::var("A1") {
        Ok(ok) => println!("[{}]", ok),
        Err(err) => println!("ERROR: [{}]", err),
    }
}
