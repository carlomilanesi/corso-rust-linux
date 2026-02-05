fn main() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::queue!(std::io::stdout(), crossterm::cursor::Hide)?;
    let mut colonna_corrente = 0;
    let numero_colonne = crossterm::terminal::size()?.0;
    let numero_righe = crossterm::terminal::size()?.1;
    loop {
        crossterm::queue!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(colonna_corrente, numero_righe - 1),
            crossterm::style::Print('*'),
        )?;
        use std::io::Write;
        std::io::stdout().flush()?;
        match crossterm::event::read()? {
            crossterm::event::Event::Key(tasto) => match tasto.code {
                crossterm::event::KeyCode::Left => {
                    if colonna_corrente > 0 {
                        colonna_corrente -= 1;
                    }
                }
                crossterm::event::KeyCode::Right => {
                    if colonna_corrente < numero_colonne - 1 {
                        colonna_corrente += 1;
                    }
                }
                crossterm::event::KeyCode::Esc => break,

                // crossterm::event::KeyCode::Char(carattere) if carattere == 'q' => {
                //     break;
                // }

                // crossterm::event::KeyCode::Char(carattere) => {
                //     if carattere == 'q' {
                //         break;
                //     }
                // }
                crossterm::event::KeyCode::Char('q') => break,
                _ => {}
            },
            _ => {}
        }
    }
    crossterm::queue!(std::io::stdout(), crossterm::cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
