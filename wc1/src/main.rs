use std::io::Read;

/*
fn imposta_rosso() -> &'static str {
    "\x1b[31m"
}

fn imposta_giallo() -> &'static str {
    "\x1b[33m"
}

fn imposta_verde() -> &'static str {
    "\x1b[32m"
}

fn reimposta_colore() -> &'static str {
    "\x1b[0m"
}
*/
use colored::Colorize;

fn main() {
    let mut argomenti = std::env::args();
    let percorso_programma = argomenti.next().unwrap();
    let mut conta_righe = false;
    let mut conta_parole = false;
    let mut conta_byte = false;
    let mut conta_caratteri = false;
    let mut percorso_file = String::new();
    for arg in argomenti {
        if arg == "-l" {
            // Si devono contare le righe
            conta_righe = true;
        } else if arg == "-w" {
            // Si devono contare le parole
            conta_parole = true;
        } else if arg == "-c" {
            // Si devono contare i byte
            conta_byte = true;
        } else if arg == "-m" {
            // Si devono contare i caratteri
            conta_caratteri = true;
        } else if arg.starts_with("-") {
            let errore = "error".red();
            let arg_giallo = arg.yellow();
            let tip = "  tip".green();
            eprintln!(
                "{errore}: unexpected argument '{arg_giallo}' found\n\
                 \n\
                 {tip}: to pass '{arg}' as a value, use '-- {arg}'\n\
                 \n\
                 Usage: {percorso_programma} [OPTION]... [FILE]...\n\
                 \n\
                 For more information, try '--help'.",
            );
            std::process::exit(1);
        } else {
            percorso_file = arg;
        }
    }
    if !conta_righe && !conta_parole && !conta_caratteri && !conta_byte {
        conta_righe = true;
        conta_parole = true;
        conta_byte = true;
    }
    let mut file = std::fs::File::open(&percorso_file).unwrap();
    let mut contenuto = String::new();
    _ = file.read_to_string(&mut contenuto);
    let mut numero_righe = 0;
    let mut numero_parole = 0;
    let mut numero_caratteri = 0;
    let mut in_parola = false;
    for carattere in contenuto.chars() {
        numero_caratteri += 1;
        if carattere == '\n' {
            numero_righe += 1;
        }
        if carattere.is_whitespace() {
            in_parola = false;
        } else {
            // Carattere stampabile
            if !in_parola {
                numero_parole += 1;
                in_parola = true;
            }
        }
    }
    if conta_righe {
        print!("{} ", numero_righe);
    }
    if conta_parole {
        print!("{} ", numero_parole);
    }
    if conta_caratteri {
        print!("{} ", numero_caratteri);
    }
    if conta_byte {
        print!("{} ", contenuto.len());
    }
    println!("{}", percorso_file);
}
