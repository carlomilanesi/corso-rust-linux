use std::io::Read;
use std::io::Write;

fn hex_dump(buffer: &Vec<u8>, mut stream: impl Write) -> Result<(), std::io::Error> {
    if buffer.is_empty() {
        return Ok(());
    }
    let mut posizione = 0;
    let mut riga_ascii = String::new();
    for &byte in buffer {
        if posizione % 16 == 0 {
            if posizione > 0 {
                writeln!(stream, " |{}|", riga_ascii)?;
                riga_ascii.clear();
            }
            write!(stream, "{:08x} ", posizione)?;
        }
        if posizione % 8 == 0 {
            write!(stream, " ")?;
        }
        write!(stream, "{:02x} ", byte)?;
        let carattere = byte as char;
        if carattere.is_ascii() && !carattere.is_ascii_control() {
            riga_ascii.push(carattere);
        } else {
            riga_ascii.push('.');
        }
        posizione += 1;
    }
    if posizione % 16 != 0 {
        write!(stream, "{}", " ".repeat((16 - posizione % 16) * 3))?;
        if posizione % 16 <= 8 {
            write!(stream, " ")?;
        }
    }
    writeln!(stream, " |{}|\n{:08x}", riga_ascii, posizione)?;
    Ok(())
}

fn main() {
    // match elabora() {
    //     Ok(()) => {}
    //     Err(err) => {
    //         eprintln!("Anomalia: {}", err);
    //     }
    // }
    _ = elabora().inspect_err(|err| eprintln!("Anomalia: {}", err));
}

fn elabora() -> Result<(), &'static str> {
    let mut argomenti = std::env::args();
    let percorso_programma = argomenti.next().ok_or("Manca il nome del programma")?;
    for arg in argomenti {
        if arg.chars().next().ok_or("Argomento vuoto")? == '-' {
            eprintln!(
                "{}: invalid option -- 'k'\n\
                 Try '{} --help' for more information.",
                percorso_programma,
                std::path::Path::new(&percorso_programma)
                    .file_name()
                    .ok_or("Percorso senza nome di file")?
                    .display(),
            );
            std::process::exit(1);
        }
        let mut file = std::fs::File::open(arg).map_err(|_| "Fallita apertura del file")?;
        let mut contenuto = Vec::new();
        file.read_to_end(&mut contenuto)
            .map_err(|_| "Fallita lettura del file")?;
        hex_dump(&contenuto, std::io::stdout()).map_err(|_| "Fallita esecuzione di hex_dump")?;
    }
    if std::env::args().count() == 1 {
        let mut contenuto = Vec::new();
        std::io::stdin()
            .read_to_end(&mut contenuto)
            .map_err(|_| "Fallita lettura di Standard Input")?;
        hex_dump(&contenuto, std::io::stdout()).map_err(|_| "Fallita esecuzione di hex_dump")?;
    }
    Ok(())
}

#[test]
fn test_vuoto() {
    let mut risultato = Vec::new();
    hex_dump(&Vec::new(), &mut risultato).unwrap();
    assert_eq!(risultato, b"");
}

#[test]
fn test_10_byte() {
    let mut risultato = Vec::new();
    hex_dump(
        &vec![b'0', b'1', b'2', b' ', b'a', b'b', b'c', b' ', b'A', b'\n'],
        &mut risultato,
    )
    .unwrap();
    assert_eq!(
        risultato,
        b"00000000  30 31 32 20 61 62 63 20  41 0a                    |012 abc A.|\n\
          0000000a\n"
    );
}

#[test]
fn test_35_byte() {
    let mut risultato = Vec::new();
    hex_dump(
        &Vec::from(b"0123456789012345678901234567890123\n"),
        &mut risultato,
    )
    .unwrap();
    assert_eq!(
        risultato,
        b"00000000  30 31 32 33 34 35 36 37  38 39 30 31 32 33 34 35  |0123456789012345|\n\
          00000010  36 37 38 39 30 31 32 33  34 35 36 37 38 39 30 31  |6789012345678901|\n\
          00000020  32 33 0a                                          |23.|\n\
          00000023\n"
    );
}

#[test]
fn test_non_ascii() {
    let mut risultato = Vec::new();
    hex_dump(&Vec::from("aà€k\n"), &mut risultato).unwrap();
    assert_eq!(
        risultato,
        b"00000000  61 c3 a0 e2 82 ac 6b 0a                           |a.....k.|\n\
          00000008\n"
    );
}

#[test]
fn test_32_byte() {
    let mut risultato = Vec::new();
    hex_dump(
        &Vec::from(b"01234567890123456789012345678901"),
        &mut risultato,
    )
    .unwrap();
    assert_eq!(
        risultato,
        b"00000000  30 31 32 33 34 35 36 37  38 39 30 31 32 33 34 35  |0123456789012345|\n\
          00000010  36 37 38 39 30 31 32 33  34 35 36 37 38 39 30 31  |6789012345678901|\n\
          00000020\n"
    );
}
