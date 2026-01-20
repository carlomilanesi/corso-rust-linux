fn scostamento<T>(base: isize, p: &T) -> isize {
    p as *const T as isize - base
}

#[allow(unconditional_recursion)]
fn conta(base: isize, n: u32) {
    println!("{n}, {:p} {}", &n, scostamento(base, &n));
    conta(base, n + 1);
}

fn main() {
    let mut base = 0;
    base = &base as *const isize as isize;
    conta(base, 0);
}
