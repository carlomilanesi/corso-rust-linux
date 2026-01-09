/*
fn display(millisecondi: Option<u64>, n_max_dischi: usize, pali: &[Vec<u8>; 3]) {
    print!("\x1b[2J");
    for pos_disco in (0..n_max_dischi).rev() {
        for pos_palo in 0..3 {
            if pos_disco < pali[pos_palo].len() {
                print!(
                    "{}{}|{}{} ",
                    " ".repeat(n_max_dischi - pali[pos_palo][pos_disco] as usize),
                    "=".repeat(pali[pos_palo][pos_disco] as usize),
                    "=".repeat(pali[pos_palo][pos_disco] as usize),
                    " ".repeat(n_max_dischi - pali[pos_palo][pos_disco] as usize),
                );
            } else {
                print!("{}|{} ", " ".repeat(n_max_dischi), " ".repeat(n_max_dischi));
            }
        }
        println!();
    }
    for _ in 0..3 {
        print!("{} ", "#".repeat(n_max_dischi * 2 + 1));
    }
    println!();
    match millisecondi {
        Some(ms) => std::thread::sleep(std::time::Duration::from_millis(ms)),
        None => {
            let mut riga = String::new();
            _ = std::io::stdin().read_line(&mut riga);
        }
    }
}
*/

fn display(millisecondi: Option<u64>, n_max_dischi: usize, pali: &[Vec<u8>; 3]) {
    let mut out = String::new();
    out += "\x1b[2J";
    for pos_disco in (0..n_max_dischi).rev() {
        for pos_palo in 0..3 {
            if pos_disco < pali[pos_palo].len() {
                out.push_str(&" ".repeat(n_max_dischi - pali[pos_palo][pos_disco] as usize));
                out += &"=".repeat(pali[pos_palo][pos_disco] as usize);
                out.push('|');
                out += &"=".repeat(pali[pos_palo][pos_disco] as usize);
                out += &" ".repeat(n_max_dischi - pali[pos_palo][pos_disco] as usize);
            } else {
                out += &" ".repeat(n_max_dischi);
                out += "|";
                out += &" ".repeat(n_max_dischi);
            }
            out += " ";
        }
        out += "\n";
    }
    for _ in 0..3 {
        out += &"#".repeat(n_max_dischi * 2 + 1);
        out += " ";
    }
    println!("{}", out);
    match millisecondi {
        Some(ms) => std::thread::sleep(std::time::Duration::from_millis(ms)),
        None => {
            _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}

fn sposta_disco(
    millisecondi: Option<u64>,
    n_max_dischi: usize,
    pali: &mut [Vec<u8>; 3],
    origine: usize,
    destinazione: usize,
) {
    let disco = pali[origine].pop().unwrap();
    pali[destinazione].push(disco);
    display(millisecondi, n_max_dischi, pali);
}

fn sposta_dischi(
    millisecondi: Option<u64>,
    n_max_dischi: usize,
    pali: &mut [Vec<u8>; 3],
    n_dischi: usize,
    origine: usize,
    appoggio: usize,
    destinazione: usize,
) {
    if n_dischi > 1 {
        sposta_dischi(
            millisecondi,
            n_max_dischi,
            pali,
            n_dischi - 1,
            origine,
            destinazione,
            appoggio,
        );
    }
    sposta_disco(millisecondi, n_max_dischi, pali, origine, destinazione);
    if n_dischi > 1 {
        sposta_dischi(
            millisecondi,
            n_max_dischi,
            pali,
            n_dischi - 1,
            appoggio,
            origine,
            destinazione,
        );
    }
}

fn main() -> Result<(), String> {
    let mut argomenti = std::env::args();
    argomenti.next();
    let n_dischi = argomenti
        .next()
        .ok_or("Manca il numero di dischi")?
        .parse::<usize>()
        .map_err(|_| "Il numero di dischi deve essere un numero intero positivo")?;
    let millisecondi = argomenti
        .next()
        .map(|ms| {
            ms.parse::<u64>().map_err(|_| {
                "I millisecondi, se specificati, devono essere un numero intero positivo o nullo"
            })
        })
        .transpose()?;
    let mut pali = [vec![], vec![], vec![]];
    for n in (1..=n_dischi).rev() {
        pali[0].push(n as u8);
    }
    display(millisecondi, n_dischi, &pali);
    sposta_dischi(millisecondi, n_dischi, &mut pali, n_dischi, 0, 1, 2);
    Ok(())
}
