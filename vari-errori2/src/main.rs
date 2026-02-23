use std::error::Error;

#[derive(Debug)]
struct ErroreInputInvalido {
    descrizione_input: String,
    causa_invalidità: String,
}

impl std::fmt::Display for ErroreInputInvalido {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "Il valore di '{}' non è valido, perché {}.",
            self.descrizione_input, self.causa_invalidità
        )?;
        Ok(())
    }
}

// impl std::fmt::Debug for ErroreInputInvalido {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(
//             formatter,
//             "ErroreInputInvalido {{ descrizione_input: {}, causa_invalidità: {} }}",
//             self.descrizione_input, self.causa_invalidità
//         )?;
//         Ok(())
//     }
// }

impl Error for ErroreInputInvalido {}

const NUMERO_MASSIMO_PARTECIPANTI: u8 = 100;

fn leggi_e_stampa_numero() -> Result<(), Box<dyn Error>> {
    println!(
        "Scrivi il numero di persone partecipanti all'iniziativa (tra 1 e {NUMERO_MASSIMO_PARTECIPANTI}): "
    );
    let mut riga = String::new();
    _ = std::io::stdin().read_line(&mut riga)?;
    let numero = riga.trim().parse::<u8>()?;
    if numero == 0 {
        return Err(Box::new(ErroreInputInvalido {
            descrizione_input: "numero di partecipanti".to_string(),
            causa_invalidità: "ha valore zero".to_string(),
        }));
    }
    if numero > NUMERO_MASSIMO_PARTECIPANTI {
        return Err(Box::new(ErroreInputInvalido {
            descrizione_input: "numero di partecipanti".to_string(),
            causa_invalidità: format!("ha valore superiore a {NUMERO_MASSIMO_PARTECIPANTI}"),
        }));
    }
    println!("Il numero formattato è {numero}.");
    Ok(())
}

fn main() {
    _ = leggi_e_stampa_numero().map_err(|err| {
        if err.downcast_ref::<std::num::ParseIntError>().is_some() {
            eprintln!("Errore di parsing del numero intero: {err}");
        } else if err.downcast_ref::<std::io::Error>().is_some() {
            eprintln!("Errore di I/O: {err}");
        } else if err.downcast_ref::<ErroreInputInvalido>().is_some() {
            eprintln!("Errore di input non valido: {err}");
        } else {
            eprintln!("Errore sconosciuto: {err}");
        }
        err
    })
}
