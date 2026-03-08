fn stampa_con_codici_escape() {
    let n = '\n';
    println!("1:{n}a\nb");
    let r = '\r';
    println!("2:{r}a\rb");
    let t = '\t';
    println!("3:{t}a\tb");
    let esadecimale = '\x4d';
    println!("4:{esadecimale}a\x4db");
    let euro = '\u{20ac}';
    let granchio = '\u{1f980}';
    let pinguino = '\u{1f427}';
    let carattere_sostitutivo = '\u{fffd}';
    println!("5:{euro}a{granchio}b{pinguino}c{carattere_sostitutivo}d\u{1f980}e");
}

fn stampa_caratteri_unicode() {
    for codice in 0..2_000_000 {
        let possibile_carattere = char::from_u32(codice);
        if possibile_carattere.is_some() {
            let carattere = possibile_carattere.unwrap();
            println!(
                "{codice:7} {codice:06x} {}{}",
                if carattere.is_control() { ":" } else { "" },
                if carattere.is_control() {
                    match codice {
                        0x00..=0x1F => char::from_u32(0x2400 + codice).unwrap(),
                        0x7F => '\u{2421}', // DEL
                        _ => '\u{00b7}',    // Punto centrale
                    }
                } else {
                    carattere
                },
            );
        }
    }
}

fn main() {
    stampa_con_codici_escape();
    stampa_caratteri_unicode();
}
