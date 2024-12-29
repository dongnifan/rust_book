use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    let n1 = add_one_v1(2);
    let n2 = add_one_v2(222);
    let n3 = add_one_v3(2);
    let n4 = add_one_v4(222);

    println!("{n1} {n2} {n3} {n4}");

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(String::from("5"));

    println!("{s} {n}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn( move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
