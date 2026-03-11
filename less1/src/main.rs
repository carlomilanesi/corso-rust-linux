use std::error::Error;
use std::io::Write;

mod app {
    use bstr::ByteSlice;
    use memmap2::Mmap;
    use std::error::Error;

    struct InfoRiga {
        inizio: usize,
        numero_paragrafo: Option<usize>,
    }

    pub struct App {
        mmap: Mmap,
        numero_prima_riga_mostrata: usize,
        altezza: usize,
        righe: Vec<InfoRiga>,
        numero_paragrafi: usize,
    }

    #[derive(Debug)]
    pub struct ErroreApp {
        pub messaggio: String,
    }

    impl Error for ErroreApp {}

    impl std::fmt::Display for ErroreApp {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            Ok(())
        }
    }

    impl App {
        pub fn nuova() -> Result<Self, Box<dyn Error>> {
            let mut argomenti = std::env::args();
            argomenti.next();
            let percorso_file = argomenti.next();
            if percorso_file.is_none() {
                return Err(Box::new(ErroreApp {
                    messaggio: "Argomento mancante\nUso: less1 <percorso file>".to_string(),
                }));
            }
            let percorso_file = percorso_file.unwrap();

            let file = std::fs::File::open(percorso_file)?;
            let mmap = unsafe { Mmap::map(&file)? };
            let dimensione_schermo = crossterm::terminal::size()?;
            let larghezza = dimensione_schermo.0 as usize;
            let altezza = dimensione_schermo.1 as usize;
            let mut numero_paragrafi = 1;
            for byte in mmap.bytes() {
                if byte == b'\n' {
                    numero_paragrafi += 1;
                }
            }
            let mut righe = vec![InfoRiga {
                inizio: 0,
                numero_paragrafo: Some(0),
            }];

            let mut numero_carattere_in_riga = 0;
            let larghezza_massima_riga = larghezza - format!("{}", numero_paragrafi).len() - 1;
            let mut indice_byte = 0;
            let mut indice_paragrafo = 0;
            for (inizio, fine, carattere) in mmap.char_indices() {
                indice_byte += fine - inizio;
                if carattere == '\n' {
                    indice_paragrafo += 1;
                    righe.push(InfoRiga {
                        inizio: indice_byte,
                        numero_paragrafo: Some(indice_paragrafo),
                    });
                    numero_carattere_in_riga = 0;
                } else {
                    numero_carattere_in_riga += 1;
                    if numero_carattere_in_riga == larghezza_massima_riga {
                        righe.push(InfoRiga {
                            inizio: indice_byte,
                            numero_paragrafo: None,
                        });
                        numero_carattere_in_riga = 0;
                    }
                }
            }
            Ok(Self {
                mmap,
                numero_prima_riga_mostrata: 0,
                altezza,
                numero_paragrafi,
                righe,
            })
        }

        pub fn numero_righe_terminale(&self) -> usize {
            self.altezza
        }
        pub fn numero_prima_riga_mostrata(&self) -> usize {
            self.numero_prima_riga_mostrata
        }
        pub fn imposta_prima_riga_mostrata(&mut self, numero_prima_riga_mostrata: usize) {
            self.numero_prima_riga_mostrata = numero_prima_riga_mostrata;
        }
        fn numero_paragrafi_testo(&self) -> usize {
            self.numero_paragrafi
        }
        pub fn numero_righe_testo(&self) -> usize {
            self.righe.len()
        }
        fn larghezza_massima_numero_paragrafo(&self) -> usize {
            format!("{}", self.numero_paragrafi_testo()).len()
        }
        fn testo_riga(&self, numero_riga_richiesto: usize) -> String {
            let inizio = self.righe[numero_riga_richiesto].inizio;
            let mut fine = if numero_riga_richiesto + 1 < self.righe.len() {
                self.righe[numero_riga_richiesto + 1].inizio
            } else {
                self.mmap.len()
            };
            if fine > inizio && self.mmap[fine - 1] == b'\n' {
                fine -= 1;
            }
            let mut risultato = String::new();
            for carattere in String::from_utf8_lossy(&self.mmap[inizio..fine]).chars() {
                risultato.push(if carattere.is_control() {
                    let codice = carattere as u32;
                    match codice {
                        0x00..=0x1f => char::from_u32(0x2400 + codice).unwrap(),
                        0x7f => '\u{2421}',
                        _ => '\u{b7}',
                    }
                } else {
                    carattere
                })
            }
            risultato
        }
        fn numero_paragrafo(&self, numero_riga_richiesto: usize) -> Option<usize> {
            self.righe[numero_riga_richiesto].numero_paragrafo
        }
        pub fn disegna(&self, out: &mut std::io::Stdout) -> std::io::Result<()> {
            crossterm::queue!(
                out,
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
            )?;
            let larghezza_numero = self.larghezza_massima_numero_paragrafo();
            for riga_schermo in 0..self.numero_righe_terminale() {
                let numero_riga_corrente = riga_schermo + self.numero_prima_riga_mostrata();
                if numero_riga_corrente >= self.numero_righe_testo() {
                    break;
                }
                crossterm::queue!(
                    out,
                    crossterm::cursor::MoveTo(0, riga_schermo as u16),
                    crossterm::style::SetForegroundColor(crossterm::style::Color::Blue)
                )?;
                match self.numero_paragrafo(numero_riga_corrente) {
                    Some(riga_file) => crossterm::queue!(
                        out,
                        crossterm::style::Print(format!("{:>larghezza_numero$} ", riga_file + 1)),
                    )?,
                    None => crossterm::queue!(
                        out,
                        crossterm::style::Print(format!("{:>larghezza_numero$} ", ' '))
                    )?,
                }
                crossterm::queue!(
                    out,
                    crossterm::style::ResetColor,
                    crossterm::style::Print(self.testo_riga(numero_riga_corrente))
                )?;
            }
            Ok(())
        }
    }
}

fn esegui(out: &mut std::io::Stdout) -> Result<(), Box<dyn Error>> {
    let mut app = app::App::nuova()?;
    let mut da_disegnare = true;
    loop {
        if da_disegnare {
            app.disegna(out)?;
        } else {
            crossterm::queue!(out, crossterm::style::Print('\x07'),)?;
        };
        out.flush()?;
        da_disegnare = false;
        match crossterm::event::read()? {
            crossterm::event::Event::Key(tasto) => match tasto.code {
                crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => break,
                crossterm::event::KeyCode::Up => {
                    if app.numero_prima_riga_mostrata() > 0 {
                        app.imposta_prima_riga_mostrata(app.numero_prima_riga_mostrata() - 1);
                        da_disegnare = true;
                    }
                }
                crossterm::event::KeyCode::Down => {
                    if app.numero_prima_riga_mostrata() + app.numero_righe_terminale()
                        < app.numero_righe_testo()
                    {
                        app.imposta_prima_riga_mostrata(app.numero_prima_riga_mostrata() + 1);
                        da_disegnare = true;
                    }
                }
                crossterm::event::KeyCode::PageUp => {
                    // app.prima_riga_mostrata -= app.numero_righe_terminale;
                    if app.numero_prima_riga_mostrata() > 0 {
                        app.imposta_prima_riga_mostrata(
                            if app.numero_prima_riga_mostrata() >= app.numero_righe_terminale() {
                                app.numero_prima_riga_mostrata() - app.numero_righe_terminale()
                            } else {
                                0
                            },
                        );
                        da_disegnare = true;
                    }
                }
                crossterm::event::KeyCode::PageDown => {
                    // app.prima_riga_mostrata += app.numero_righe_terminale;
                    let righe_testo_dopo_prima_riga_mostrata =
                        app.numero_righe_testo() - app.numero_prima_riga_mostrata();
                    if righe_testo_dopo_prima_riga_mostrata > app.numero_righe_terminale() {
                        let righe_non_visibili_in_basso =
                            righe_testo_dopo_prima_riga_mostrata - app.numero_righe_terminale();
                        app.imposta_prima_riga_mostrata(
                            app.numero_prima_riga_mostrata()
                                + if righe_non_visibili_in_basso > app.numero_righe_terminale() {
                                    app.numero_righe_terminale()
                                } else {
                                    righe_non_visibili_in_basso
                                },
                        );
                        da_disegnare = true;
                    }
                }
                crossterm::event::KeyCode::Home => {
                    if app.numero_prima_riga_mostrata() > 0 {
                        app.imposta_prima_riga_mostrata(0);
                        da_disegnare = true;
                    }
                }
                crossterm::event::KeyCode::End => {
                    if app.numero_prima_riga_mostrata() + app.numero_righe_terminale()
                        < app.numero_righe_testo()
                    {
                        app.imposta_prima_riga_mostrata(
                            app.numero_righe_testo() - app.numero_righe_terminale(),
                        );
                        da_disegnare = true;
                    }
                }
                _ => {}
            },
            crossterm::event::Event::Resize(_, _) => {
                app = app::App::nuova()?;
                da_disegnare = true;
            }
            _ => {}
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // let mut vettore = Vec::from("aèb".as_bytes());
    // vettore.push(b'\x80');
    // // for b in "€".as_bytes() {
    // //     vettore.push(*b);
    // // }
    // vettore.extend("€".as_bytes());
    // println!("{:?} {}", vettore, vettore.len());
    // println!("{:?}", String::from_utf8(vettore.clone()));
    // let stringa = String::from_utf8_lossy(&vettore);
    // println!("{} {}", stringa, stringa.chars().count());
    // for (indice, carattere) in stringa.char_indices() {
    //     print!("{indice} {carattere}; ",);
    // }
    // println!("{}", stringa.len());
    // for indice_carattere in stringa.char_indices() {
    //     print!("{} {}; ", indice_carattere.0, indice_carattere.1);
    // }
    // println!("{}", stringa.len());
    // println!("{}", char::REPLACEMENT_CHARACTER.len_utf8());

    // use bstr::ByteSlice;
    // for (inizio, fine, carattere) in vettore.char_indices() {
    //     print!("{inizio} {fine} {carattere}; ",);
    // }
    // println!("{}", vettore.len());
    // return Ok(());

    let mut out = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::queue!(
        out,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    let risultato = esegui(&mut out);
    crossterm::queue!(
        out,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    out.flush()?;
    crossterm::terminal::disable_raw_mode()?;
    match risultato {
        Err(err) => {
            let percorso_programma = std::env::args().next().unwrap();
            let errore_app = err.downcast_ref::<app::ErroreApp>();
            if errore_app.is_some() {
                eprintln!("{percorso_programma}: {}", errore_app.unwrap().messaggio);
            } else {
                let errore_io = err.downcast_ref::<std::io::Error>();
                if errore_io.is_some() {
                    eprintln!("{percorso_programma}: {}", errore_io.unwrap());
                } else {
                    eprintln!("{percorso_programma}: {:?}", err);
                }
            }
            std::process::exit(1);
        }
        Ok(_) => {}
    }
    Ok(())
}
