fn is_equal<T: Eq>(t1: &T, t2: &T) -> bool {
    t1 == t2
  }

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

    println!("{}", is_equal(&s1, &s2));
}
