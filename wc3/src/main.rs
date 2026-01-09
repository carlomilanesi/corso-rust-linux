fn main() {
    let istante_inizio = std::time::Instant::now();
    let mut argomenti = std::env::args();
    argomenti.next();
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
        } else {
            percorso_file = arg;
        }
    }
    if !conta_righe && !conta_parole && !conta_caratteri && !conta_byte {
        conta_righe = true;
        conta_parole = true;
        conta_byte = true;
    }
    let file = std::fs::File::open(&percorso_file).unwrap();
    let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
    let mut numero_righe = 0;
    let mut numero_parole = 0;
    let mut numero_caratteri = 0;
    let mut in_parola = false;
    for carattere in std::str::from_utf8(&mmap).unwrap().chars() {
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
        print!("{} ", mmap.len());
    }
    println!("{}", percorso_file);
    println!("{:?}", istante_inizio.elapsed());
}
