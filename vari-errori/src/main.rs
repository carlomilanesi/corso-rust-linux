// //// 1. Non gestisce errori. ////

fn leggi_e_stampa_numero() {
    println!("Scrivi un numero: ");
    let mut riga = String::new();
    _ = std::io::stdin().read_line(&mut riga);
    let numero = riga.trim().parse::<f64>().unwrap();
    println!("Il numero formattato è {numero}.");
}

fn main() {
    leggi_e_stampa_numero();
}

//// 2. Gestisce errori di I/O, ma non errori di parsing di float. ////

// fn leggi_e_stampa_numero() -> Result<(), std::io::Error> {
//     println!("Scrivi un numero: ");
//     let mut riga = String::new();
//     _ = std::io::stdin().read_line(&mut riga)?;
//     let numero = riga.trim().parse::<f64>().unwrap();
//     println!("Il numero formattato è {numero}.");
//     Ok(())
// }

// fn main() {
//     _ = leggi_e_stampa_numero().or_else(|err| {
//         eprintln!("Errore di I/O: {err}");
//         Err(err)
//     });
// }

//// 3. Gestisce errori di parsing di float, ma non errori di I/O. ////

// fn leggi_e_stampa_numero() -> Result<(), std::num::ParseFloatError> {
//     println!("Scrivi un numero: ");
//     let mut riga = String::new();
//     _ = std::io::stdin().read_line(&mut riga);
//     let numero = riga.trim().parse::<f64>()?;
//     println!("Il numero formattato è {numero}.");
//     Ok(())
// }

// fn main() {
//     _ = leggi_e_stampa_numero().or_else(|err| {
//         eprintln!("Errore di parsing del numero: {err}");
//         Err(err)
//     });
// }

//// 4.a Gestisce sia errori di I/O, sia di parsing di float. ////
//// Gestione errori caotica. ////

// fn leggi_e_stampa_numero() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Scrivi un numero: ");
//     let mut riga = String::new();
//     _ = std::io::stdin().read_line(&mut riga)?;
//     let numero = riga.trim().parse::<f64>()?;
//     println!("Il numero formattato è {numero}.");
//     Ok(())
// }

// fn main() {
//     _ = leggi_e_stampa_numero().or_else(|err| {
//         let e1 = err.downcast_ref::<std::num::ParseFloatError>();
//         if e1.is_some() {
//             eprintln!("Errore di parsing del numero: {err}");
//         } else {
//             let e2 = err.downcast_ref::<std::io::Error>();
//             if e2.is_some() {
//                 eprintln!("Errore di I/O: {err}");
//             } else {
//                 eprintln!("Errore sconosciuto: {err}");
//             }
//         }
//         Err(err)
//     });
// }

//// 4.b Gestisce sia errori di I/O, sia di parsing di float. ////
//// Gestione errori ordinata, con uso di `is_some`. ////

// fn main() {
//     _ = leggi_e_stampa_numero().or_else(|err| {
//         if err.downcast_ref::<std::num::ParseFloatError>().is_some() {
//             eprintln!("Errore di parsing del numero: {err}");
//         } else if err.downcast_ref::<std::io::Error>().is_some() {
//             eprintln!("Errore di I/O: {err}");
//         } else {
//             eprintln!("Errore sconosciuto: {err}");
//         }
//         Err(err)
//     });
// }
