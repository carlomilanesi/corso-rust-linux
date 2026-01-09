fn elabora(mut first_number: f64, mut second_number: f64, mut third_number: f64) {
    first_number = first_number * 1.0e9;
    second_number = second_number * 1e9;
    third_number = third_number * 1e9;
    let mut n = first_number;
    loop {
        if n > third_number {
            return;
        }
        println!("{}", n / 1e9);
        n = n + second_number;
    }
}

fn main() {
    let mut i_miei_argomenti = std::env::args();
    _ = i_miei_argomenti.next();
    let first = i_miei_argomenti.next();
    let second = i_miei_argomenti.next();
    let third = i_miei_argomenti.next();
    let possible_first_number;
    if first.is_some() {
        possible_first_number = first.unwrap().parse::<f64>();
    } else {
        return;
    }

    let first_number;
    if possible_first_number.is_ok() {
        first_number = possible_first_number.unwrap();
    } else {
        return;
    }

    let possible_second_number;
    if second.is_some() {
        possible_second_number = second.unwrap().parse::<f64>();
    } else {
        elabora(1., 1., first_number);
        return;
    }

    let second_number;
    if possible_second_number.is_ok() {
        second_number = possible_second_number.unwrap();
    } else {
        return;
    }

    let possible_third_number;
    if third.is_some() {
        possible_third_number = third.unwrap().parse::<f64>();
    } else {
        elabora(first_number, 1., second_number);
        return;
    }

    let third_number;
    if possible_third_number.is_ok() {
        third_number = possible_third_number.unwrap();
    } else {
        return;
    }

    elabora(first_number, second_number, third_number);
}
