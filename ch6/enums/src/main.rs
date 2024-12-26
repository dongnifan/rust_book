fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match & opt {
        // _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}
