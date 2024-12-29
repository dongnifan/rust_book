fn main() {
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }

    for n_ref in &v {
        // n_ref has type &i32
        let v = *n_ref;
        println!("{v}");
    }
}
