use std::mem::size_of; // dimensione di
use std::mem::size_of_val; // dimensione di valore

fn mostra_byte<T>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const _ as *const u8, size_of::<T>()) }
}

fn main() {
    // Uso di `size_of` sui tipi primitivi
    println!("i8: size={}, da {} a {}", size_of::<i8>(), i8::MIN, i8::MAX);
    println!("u8: size={}, da {} a {}", size_of::<u8>(), u8::MIN, u8::MAX);
    println!(
        "i16: size={}, da {} a {}",
        size_of::<i16>(),
        i16::MIN,
        i16::MAX,
    );
    println!(
        "u16: size={}, da {} a {}",
        size_of::<u16>(),
        u16::MIN,
        u16::MAX,
    );
    println!(
        "i32: size={}, da {} a {}",
        size_of::<i32>(),
        i32::MIN,
        i32::MAX,
    );
    println!(
        "u32: size={}, da {} a {}",
        size_of::<u32>(),
        u32::MIN,
        u32::MAX,
    );
    println!(
        "i64: size={}, da {} a {}",
        size_of::<i64>(),
        i64::MIN,
        i64::MAX,
    );
    println!(
        "u64: size={}, da {} a {}",
        size_of::<u64>(),
        u64::MIN,
        u64::MAX,
    );
    println!(
        "i128: size={}, da {} a {}",
        size_of::<i128>(),
        i128::MIN,
        i128::MAX,
    );
    println!(
        "u128: size={}, da {} a {}",
        size_of::<u128>(),
        u128::MIN,
        u128::MAX,
    );
    println!(
        "isize: size={}, da {} a {}",
        size_of::<isize>(),
        isize::MIN,
        isize::MAX,
    );
    println!(
        "usize: size={}, da {} a {}",
        size_of::<usize>(),
        usize::MIN,
        usize::MAX,
    );
    println!(
        "f32: size={}, da {} a {}",
        size_of::<f32>(),
        f32::MIN,
        f32::MAX,
    );
    println!(
        "f64: size={}, da {} a {}",
        size_of::<f64>(),
        f64::MIN,
        f64::MAX,
    );
    println!("bool: size={}", size_of::<bool>());
    println!(
        "char: size={}, da {} a {}",
        size_of::<char>(),
        char::MIN,
        char::MAX,
    );

    // Uso di `size_of` su riferimenti e array
    println!("&u8: size={}", size_of::<&u8>());
    println!("&i128: size={}", size_of::<&i128>());
    //println!("str: size={}", size_of::<str>());
    //println!("[u8]: size={}", size_of::<[u8]>());
    println!("[u32; 3]: size={}", size_of::<[u32; 3]>());
    println!("&[u8]: size={}", size_of::<&[u8]>());
    println!("&[i128]: size={}", size_of::<&[i128]>());
    println!("&str: size={}", size_of::<&str>());

    // Dichiarazione di variabili di vari tipi
    let v_i8: i8 = 1i8;
    let v_u8: u8 = 2u8;
    let v_i16: i16 = 3i16;
    let v_u16: u16 = 4u16;
    let v_i32: i32 = 5i32;
    let v_u32: u32 = 6u32;
    let v_i64: i64 = 7i64;
    let v_u64: u64 = 8u64;
    let v_i128: i128 = 9i128;
    let v_u128: u128 = 10u128;
    let v_isize: isize = 11isize;
    let v_usize: usize = 12usize;
    let v_f32: f32 = 13f32;
    let v_f64: f64 = 14f64;
    let v_r_u8: &u8 = &v_u8;
    let v_r_i128: &i128 = &v_i128;
    let v_array1: [u32; 3] = [15u32; 3];
    let v_array2: [u32; 3] = [16u32, 17u32, 18u32];
    let v_r_array = &v_array1;
    let v_r_str: &str = "abcd€";

    // Uso di `size_of_val` sulle variabili dichiarate
    println!();
    println!("i8: size={}", size_of_val(&v_i8));
    println!("u8: size={}", size_of_val(&v_u8));
    println!("i16: size={}", size_of_val(&v_i16));
    println!("u16: size={}", size_of_val(&v_u16));
    println!("i32: size={}", size_of_val(&v_i32));
    println!("u32: size={}", size_of_val(&v_u32));
    println!("i64: size={}", size_of_val(&v_i64));
    println!("u64: size={}", size_of_val(&v_u64));
    println!("i128: size={}", size_of_val(&v_i128));
    println!("u128: size={}", size_of_val(&v_u128));
    println!("isize: size={}", size_of_val(&v_isize));
    println!("usize: size={}", size_of_val(&v_usize));
    println!("f32: size={}", size_of_val(&v_f32));
    println!("f64: size={}", size_of_val(&v_f64));
    println!("&u8: size={}", size_of_val(&v_r_u8));
    println!("&i128: size={}", size_of_val(&v_r_i128));
    println!("[u32; 3]: size={}", size_of_val(&v_array1));
    println!("[u32; 3]: size={}", size_of_val(&v_array2));
    println!("&[u32; 3]: size={}", size_of_val(&v_r_array));
    println!("&str: size={}", size_of_val(&v_r_str));

    println!();
    println!("valore di v_i8:\t{} {}", v_i8, &v_i8);

    println!(
        "v_i8\tindirizzo: {:p}, valore: {:?}",
        &v_i8,
        mostra_byte(&v_i8)
    );
    println!(
        "v_u8\tindirizzo: {:p}, valore: {:?}",
        &v_u8,
        mostra_byte(&v_u8)
    );
    println!(
        "v_i16\tindirizzo: {:p}, valore: {:?}",
        &v_i16,
        mostra_byte(&v_i16)
    );
    println!(
        "v_u16\tindirizzo: {:p}, valore: {:?}",
        &v_u16,
        mostra_byte(&v_u16)
    );
    println!(
        "v_i32\tindirizzo: {:p}, valore: {:?}",
        &v_i32,
        mostra_byte(&v_i32),
    );
    println!(
        "v_u32\tindirizzo: {:p}, valore: {:?}",
        &v_u32,
        mostra_byte(&v_u32),
    );
    println!(
        "v_i64\tindirizzo: {:p}, valore: {:?}",
        &v_i64,
        mostra_byte(&v_i64),
    );
    println!(
        "v_u64\tindirizzo: {:p}, valore: {:?}",
        &v_u64,
        mostra_byte(&v_u64),
    );
    println!(
        "v_i128\tindirizzo: {:p}, valore: {:?}",
        &v_i128,
        mostra_byte(&v_i128),
    );
    println!(
        "v_u128\tindirizzo: {:p}, valore: {:?}",
        &v_u128,
        mostra_byte(&v_u128),
    );
    println!(
        "v_isize\tindirizzo: {:p}, valore: {:?}",
        &v_isize,
        mostra_byte(&v_isize),
    );
    println!(
        "v_usize\tindirizzo: {:p}, valore: {:?}",
        &v_usize,
        mostra_byte(&v_usize),
    );
    println!(
        "v_f32\tindirizzo: {:p}, valore: {:?}",
        &v_f32,
        mostra_byte(&v_f32),
    );
    println!(
        "v_f64\tindirizzo: {:p}, valore: {:?}",
        &v_f64,
        mostra_byte(&v_f64),
    );

    // indirizzo = address
    // puntatore = pointer
    // riferimento = reference
    // prendere in prestito = borrowing
    println!("indirizzo di v_i8:\t{:p}", &v_i8);
    //let indirizzo: u64 = &v_i8; // ILLEGALE
    //let indirizzo: usize = &v_i8; // ILLEGALE
    let indirizzo: usize = &v_i8 as *const i8 as usize;
    println!("indirizzo di v_i8: dec: {indirizzo}, hex: {indirizzo:x}");

    println!("v_r_u8  \tvalore: {:?}", mostra_byte(&v_r_u8),);
    println!("v_r_i128\tvalore: {:?}", mostra_byte(&v_r_i128),);
    println!("v_array1\tvalore: {:?}", mostra_byte(&v_array1),);
    println!("v_array2\tvalore: {:?}", mostra_byte(&v_array2),);
    println!("v_r_array\tvalore: {:?}", mostra_byte(&v_r_array),);
    println!("v_r_str \tvalore: {:?}", mostra_byte(&v_r_str),);
}
