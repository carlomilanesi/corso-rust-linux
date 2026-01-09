fn main() {
    let mut argomenti = std::env::args();
    argomenti.next();
    for arg in argomenti {
        match std::fs::remove_file(&arg) {
            Ok(_) => {
                println!("File '{}' eliminato.", arg);
            }
            Err(errore) => {
                println!(
                    "Eliminazione del file '{}' fallita, a causa di '{} ({})'.",
                    arg,
                    errore,
                    errore.kind(),
                );
            }
        }
    }
}
