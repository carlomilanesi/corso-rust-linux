use std::io::Write;

fn main() {
    let mut argomenti = std::env::args();
    _ = argomenti.next();
    let mut contatore = argomenti.next().unwrap().parse::<u32>().unwrap();
    dbg!(contatore);
    while contatore != 0 {
        print!("\r{}      ", contatore);
        _ = std::io::stdout().flush();
        std::thread::sleep(std::time::Duration::from_secs(1));
        contatore = contatore - 1;
    }
    println!("\rFatto!   ");
}
