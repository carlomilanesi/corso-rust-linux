use std::process::Stdio;

fn main() {
    let mut argomenti = std::env::args();
    argomenti.next();
    for percorso in argomenti {
        let processo_hd = std::process::Command::new("hd")
            .arg("-v")
            .arg(&percorso)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let processo_wc = std::process::Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(processo_hd.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let output_wc = processo_wc.wait_with_output().unwrap();
        print!(
            "Numero righe: {}",
            std::str::from_utf8(&output_wc.stdout).unwrap()
        );
    }
}
