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

    let a = 11_u32;
    let mut b = 12_u32;
    // a = 13_u32; // ILLEGALE
    b = 14_u32;
    println!("{}", a); // 11
    // println!("{:p}", a); // ILLEGALE
    println!("{}", &a); // 11
    println!("{:p}", &a); // 0x...
    // println!("{}", *a); // ILLEGALE
    // println!("{:p}", *a); // ILLEGALE
    println!("{}", *&a); // 11

    println!("{:p}", &7); // 0x...
    println!("{:p}", &(a + b / 2)); // 0x...

    let r_a = &a;
    let r_15 = &15;
    println!("{} {} {} {} {}", a, r_a, r_15, *r_a, *r_15); // 11, 11, 15, 11, 15

    // let r_a_m = &mut a; // ILLEGALE
    let r_15_m = &mut 15;
    *r_15_m = 16;
    let _r_b_m = &mut b;
    let _r_b_i = &b;
    // _r_b_m = &mut b; // ILLEGALE
    // _r_b_i = &b; // ILLEGALE
    let mut _r_m_b_m = &mut b;
    let mut _r_m_b_i = &b;
    _r_m_b_m = &mut b;
    _r_m_b_i = &b;

    let box_i: Box<i16> = Box::new(300_i16);
    println!(
        "box_i:{}, box_i:{:p}, &box_i:{}, &box_i:{:p}, *box_i:{}",
        box_i, box_i, &box_i, &box_i, *box_i
    );
    //*box_i = 301; // ILLEGALE
    //box_i = box_i; // ILLEGALE

    let mut box_m: Box<i16> = Box::new(400_i16);
    println!(
        "box_m:{}, box_m:{:p}, &box_m:{}, &box_m:{:p}, *box_m:{}",
        box_m, box_m, &box_m, &box_m, *box_m
    );
    *box_m = 401;
    box_m = box_i;
}

fn main() {
    //mostra_stringa();
    //mostra_vettore();
    mostra_box();
}
