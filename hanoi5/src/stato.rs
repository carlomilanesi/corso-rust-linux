pub struct Stato {
    millisecondi: Option<u64>,
    n_max_dischi: usize,
    pali: [Vec<u8>; 3],
}

impl Stato {
    pub fn nuovo(millisecondi: Option<u64>, n_max_dischi: usize) -> Self {
        let mut primo_palo = vec![];
        for n in (1..=n_max_dischi).rev() {
            primo_palo.push(n as u8);
        }
        Self {
            millisecondi,
            n_max_dischi,
            pali: [primo_palo, vec![], vec![]],
        }
    }

    pub fn display(&self) {
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

    pub fn sposta_dischi(
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
