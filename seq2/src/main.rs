fn elabora(mut primo_numero: f64, mut secondo_numero: f64, mut terzo_numero: f64) {
    primo_numero *= 1.0e9;
    secondo_numero *= 1e9;
    terzo_numero *= 1e9;
    let mut n = primo_numero;
    loop {
        if n > terzo_numero {
            return;
        }
        println!("{}", n / 1e9);
        n += secondo_numero;
    }
}

fn main() {
    let risultato_di_esegui = esegui();
    if risultato_di_esegui.is_err() {
        let err = risultato_di_esegui.unwrap_err();
        let percorso_programma = std::env::args().next().unwrap();
        eprintln!(
            "{percorso_programma}: {err}\n\
             Try '{percorso_programma} --help' for more information."
        );
        std::process::exit(1);
    }
}

fn esegui() -> Result<(), String> {
    let mut i_miei_argomenti = std::env::args();
    i_miei_argomenti.next();
    let primo_argomento = i_miei_argomenti.next().ok_or("missing operand")?;
    let primo_numero = primo_argomento
        .parse::<f64>()
        .map_err(|_| format!("invalid floating point argument: '{primo_argomento}'"))?;
    let possibile_secondo_argomento = i_miei_argomenti.next();
    let possibile_terzo_argomento = i_miei_argomenti.next();
    let possibile_quarto_argomento = i_miei_argomenti.next();
    if possibile_quarto_argomento.is_some() {
        let quarto_argomento = possibile_quarto_argomento.unwrap();
        return Err(format!(
            "unexpected value '{quarto_argomento}' for '[numbers]...' found; no more were expected\n\
        \n\
        Usage: seq [OPTION]... LAST\n       \
               seq [OPTION]... FIRST LAST\n       \
               seq [OPTION]... FIRST INCREMENT LAST\n\
        "
        ));
    }
    if possibile_secondo_argomento.is_none() {
        elabora(1., 1., primo_numero);
        return Ok(());
    }
    let secondo_argomento = possibile_secondo_argomento.unwrap();
    let secondo_numero = secondo_argomento
        .parse::<f64>()
        .map_err(|_| format!("invalid floating point argument: '{secondo_argomento}'"))?;
    if possibile_terzo_argomento.is_none() {
        elabora(primo_numero, 1., secondo_numero);
        return Ok(());
    }
    let terzo_argomento = possibile_terzo_argomento.unwrap();
    let terzo_numero = terzo_argomento
        .parse::<f64>()
        .map_err(|_| format!("invalid floating point argument: '{terzo_argomento}'"))?;
    elabora(primo_numero, secondo_numero, terzo_numero);
    Ok(())
}
