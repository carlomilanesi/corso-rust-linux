//! Programma che mostra l'uso
//! dello stack
//! nel calcolo del fattoriale.

fn scostamento<T>(base: isize, p: &T) -> isize {
    p as *const T as isize - base
}

/** Restituisce
 * il fattoriale
 * dell'argomento `n`.
 */
fn fattoriale(base: isize, valori: &mut Vec<(f32, f32)>, n: u32) -> u128 {
    let a = n;
    let b = n;
    valori.push((valori.len() as f32, scostamento(base, &base) as f32));
    valori.push((valori.len() as f32, scostamento(base, &valori) as f32));
    valori.push((valori.len() as f32, scostamento(base, &n) as f32));
    valori.push((valori.len() as f32, scostamento(base, &a) as f32));
    valori.push((valori.len() as f32, scostamento(base, &b) as f32));
    if n <= 1 {
        1
    } else {
        n as u128 * fattoriale(base, valori, n - 1)
    }
}

fn main() {
    let n = 5;
    let base = &n as *const u32 as isize;
    let mut valori = vec![(0f32, scostamento(base, &n) as f32)];
    let fatt;
    valori.push((valori.len() as f32, scostamento(base, &base) as f32));
    valori.push((valori.len() as f32, scostamento(base, &valori) as f32));
    fatt = fattoriale(base, &mut valori, n);
    valori.push((valori.len() as f32, scostamento(base, &fatt) as f32));
    println!("fattoriale({n})={fatt}",);
    _ = use_plotters(valori);
}

// In Ubuntu, run:
// sudo apt install pkg-config libfreetype6-dev libfontconfig1-dev
// Show with the command: cargo run && xdg-open memoria.svg

use plotters::prelude::*;

/// Usa il package Plotters per generare un file SVG
/// che descrive i `valori` ricevuti come argomento.
fn use_plotters(valori: Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    //let root = BitMapBackend::new("3.png", (300, 200)).into_drawing_area();
    let root = SVGBackend::new("memoria.svg", (300, 200)).into_drawing_area();
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart =
        ChartBuilder::on(&root).build_cartesian_2d(0f32..valori.len() as f32, -1000f32..50f32)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(PointSeries::of_element(valori, 0, &RED, &|c, _, st| {
        return EmptyElement::at(c)
            + Rectangle::new([(0, 0), (5, 1)], st.filled())
            + Text::new(format!("{}", c.1), (-2, -6), ("sans-serif", 5).into_font());
    }))?;
    root.present()?;
    Ok(())
}
