struct Stato {
    millisecondi: Option<u64>,
    n_max_dischi: usize,
    pali: [Vec<u8>; 3],
}

impl Stato {
    fn new(millisecondi: Option<u64>, n_max_dischi: usize) -> Self {
        let mut stato = Stato {
            millisecondi,
            n_max_dischi,
            pali: [vec![], vec![], vec![]],
        };
        for n in (1..=stato.n_max_dischi).rev() {
            stato.pali[0].push(n as u8);
        }
        stato
    }

    fn display(&self) {
        let mut out = String::new();
        out += "\x1b[2J";
        for pos_disco in (0..self.n_max_dischi).rev() {
            for pos_palo in 0..3 {
                if pos_disco < self.pali[pos_palo].len() {
                    out.push_str(
                        &" ".repeat(self.n_max_dischi - self.pali[pos_palo][pos_disco] as usize),
                    );
                    out += &"=".repeat(self.pali[pos_palo][pos_disco] as usize);
                    out.push('|');
                    out += &"=".repeat(self.pali[pos_palo][pos_disco] as usize);
                    out += &" ".repeat(self.n_max_dischi - self.pali[pos_palo][pos_disco] as usize);
                } else {
                    out += &" ".repeat(self.n_max_dischi);
                    out += "|";
                    out += &" ".repeat(self.n_max_dischi);
                }
                out += " ";
            }
            out += "\n";
        }
        for _ in 0..3 {
            out += &"#".repeat(self.n_max_dischi * 2 + 1);
            out += " ";
        }
        println!("{}", out);
        match self.millisecondi {
            Some(ms) => std::thread::sleep(std::time::Duration::from_millis(ms)),
            None => {
                _ = std::io::stdin().read_line(&mut String::new());
            }
        }
    }

    fn sposta_disco(&mut self, origine: usize, destinazione: usize) {
        let disco = self.pali[origine].pop().unwrap();
        self.pali[destinazione].push(disco);
        self.display();
    }

    fn sposta_dischi(
        &mut self,
        n_dischi: usize,
        origine: usize,
        appoggio: usize,
        destinazione: usize,
    ) {
        if n_dischi > 1 {
            self.sposta_dischi(n_dischi - 1, origine, destinazione, appoggio);
        }
        self.sposta_disco(origine, destinazione);
        if n_dischi > 1 {
            self.sposta_dischi(n_dischi - 1, appoggio, origine, destinazione);
        }
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
    let ms = argomenti
        .next()
        .map(|ms|
            ms.parse::<u64>().map_err(
                |_| "I millisecondi, se specificati, devono essere un numero intero positivo o nullo",
            )
        )
        .transpose()?;
    let mut stato = Stato::new(ms, n_dischi);
    stato.display();
    let n_dischi = stato.n_max_dischi;
    stato.sposta_dischi(n_dischi, 0, 1, 2);
    Ok(())
}
