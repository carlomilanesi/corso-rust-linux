fn mostra_byte<T>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const _ as *const u8, size_of::<T>()) }
}

fn mostra_stringa() {
    println!("---- mostra_stringa ----");
    let mut s: String = String::new();
    for carattere in 'A'..'N' {
        println!("s:{s}");
        println!("  byte:{:?}", mostra_byte(&s));
        println!(
            "  capacità:{}, puntatore:{:p}, lunghezza in byte:{}, contenuto in byte:{:?}",
            s.capacity(),
            s.as_ptr(),
            s.len(),
            unsafe { std::slice::from_raw_parts(s.as_ptr(), s.capacity()) },
        );
        s.push(carattere);
    }
    println!("    s:{s}");
    s.clear();
    println!(
        "  capacità:{}, puntatore:{:p}, lunghezza in byte:{}, contenuto in byte:{:?}",
        s.capacity(),
        s.as_ptr(),
        s.len(),
        unsafe { std::slice::from_raw_parts(s.as_ptr(), s.capacity()) },
    );
}

fn mostra_vettore() {
    println!("---- mostra_vettore ----");
    let mut v: Vec<u16> = Vec::new();
    for numero in 1024..1042 {
        println!("v:{v:?}");
        println!("  byte:{:?}", mostra_byte(&v));
        println!(
            "  capacità in elementi:{}, puntatore:{:p}, lunghezza in elementi:{}, contenuto in elementi:{:?}",
            v.capacity(),
            v.as_ptr(),
            v.len(),
            unsafe { std::slice::from_raw_parts(v.as_ptr(), v.capacity()) },
        );
        v.push(numero);
    }
    println!("    v:{v:?}");
    v.clear();
    println!(
        "  capacità in elementi:{}, puntatore:{:p}, lunghezza in elementi:{}, contenuto in elementi:{:?}",
        v.capacity(),
        v.as_ptr(),
        v.len(),
        unsafe { std::slice::from_raw_parts(v.as_ptr(), v.capacity()) },
    );
}

fn mostra_box() {
    println!("---- mostra_box ----");
    let mut a = 12;
    let mut r_a = &mut a;
    *r_a = 13;
    r_a = &mut a;
    let mut b: Box<u128> = Box::new(300);
    println!("b:{}, b:{:p}, &b:{}, &b:{:p}, *b:{}", b, b, &b, &b, *b);
    *b = 301;
    println!(
        "b:{}, b:{:p}, &b:{}, &b:{:p}, *b:{}, *r_a={}",
        b, b, &b, &b, *b, *r_a
    );
}

fn main() {
    //mostra_stringa();
    //mostra_vettore();
    mostra_box();
}
