struct Stato {
    millisecondi: Option<u64>,
    n_max_dischi: usize,
    pali: [Vec<u8>; 3],
}

fn display(s: &Stato) {
    let mut out = String::new();
    out += "\x1b[2J";
    for pos_disco in (0..s.n_max_dischi).rev() {
        for pos_palo in 0..3 {
            if pos_disco < s.pali[pos_palo].len() {
                out.push_str(&" ".repeat(s.n_max_dischi - s.pali[pos_palo][pos_disco] as usize));
                out += &"=".repeat(s.pali[pos_palo][pos_disco] as usize);
                out.push('|');
                out += &"=".repeat(s.pali[pos_palo][pos_disco] as usize);
                out += &" ".repeat(s.n_max_dischi - s.pali[pos_palo][pos_disco] as usize);
            } else {
                out += &" ".repeat(s.n_max_dischi);
                out += "|";
                out += &" ".repeat(s.n_max_dischi);
            }
            out += " ";
        }
        out += "\n";
    }
    for _ in 0..3 {
        out += &"#".repeat(s.n_max_dischi * 2 + 1);
        out += " ";
    }
    println!("{}", out);
    match s.millisecondi {
        Some(ms) => std::thread::sleep(std::time::Duration::from_millis(ms)),
        None => {
            _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}

fn sposta_disco(stato: &mut Stato, origine: usize, destinazione: usize) {
    let disco = stato.pali[origine].pop().unwrap();
    stato.pali[destinazione].push(disco);
    display(stato);
}

fn sposta_dischi(
    s: &mut Stato,
    n_dischi: usize,
    origine: usize,
    appoggio: usize,
    destinazione: usize,
) {
    if n_dischi > 1 {
        sposta_dischi(s, n_dischi - 1, origine, destinazione, appoggio);
    }
    sposta_disco(s, origine, destinazione);
    if n_dischi > 1 {
        sposta_dischi(s, n_dischi - 1, appoggio, origine, destinazione);
    }
}

fn main() -> Result<(), String> {
    /*
    let s = "ab";
    println!("{} {}", s.len(), str::len(s));
    let mut v = vec![1_000_000, 20, 30, 40];
    println!("{}", v.len());
    v.push(500);
    Vec::push(&mut v, 600);
    println!("{} {}", v.len(), Vec::len(&v));
    return Ok(());
    */

    let mut stato = Stato {
        millisecondi: None,
        n_max_dischi: 0,
        pali: [vec![], vec![], vec![]],
    };

    let mut argomenti = std::env::args();
    argomenti.next();
    stato.n_max_dischi = argomenti
        .next()
        .ok_or("Manca il numero di dischi")?
        .parse::<usize>()
        .map_err(|_| "Il numero di dischi deve essere un numero intero positivo")?;
    stato.millisecondi = argomenti
        .next()
        .map(|ms|
            ms.parse::<u64>().map_err(
                |_| "I millisecondi, se specificati, devono essere un numero intero positivo o nullo",
            )
        )
        .transpose()?;
    for n in (1..=stato.n_max_dischi).rev() {
        stato.pali[0].push(n as u8);
    }
    display(&stato);
    let n_dischi = stato.n_max_dischi;
    sposta_dischi(&mut stato, n_dischi, 0, 1, 2);
    Ok(())
}
