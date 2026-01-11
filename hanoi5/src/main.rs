mod stato;
use stato::Stato;

fn main() -> Result<(), String> {
    let mut argomenti = std::env::args();
    argomenti.next();
    let n_dischi = argomenti
        .next()
        .ok_or("Manca il numero di dischi")?
        .parse::<usize>()
        .map_err(|_| "Il numero di dischi deve essere un numero intero positivo")?;
    let ms = argomenti
        .next()
        .map(|ms|
            ms.parse::<u64>().map_err(
                |_| "I millisecondi, se specificati, devono essere un numero intero positivo o nullo",
            )
        )
        .transpose()?;

    /*
    let mut primo_palo = vec![];
    for n in (1..=n_dischi).rev() {
        primo_palo.push(n as u8);
    }
    let mut stato = Stato {
        millisecondi: ms,
        n_max_dischi: n_dischi,
        pali: [primo_palo, vec![], vec![]],
    };
    */
    let mut stato = Stato::nuovo(ms, n_dischi);

    stato.display();
    stato.sposta_dischi(n_dischi, 0, 1, 2);
    Ok(())
}
