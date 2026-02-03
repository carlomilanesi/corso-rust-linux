fn main() {
    match hd4::elabora() {
        Ok(_) => {}
        Err(err) => eprintln!("Errore: {err}."),
    }
}
