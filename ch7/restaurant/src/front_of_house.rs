pub mod hosting;

mod serving {
    fn take_order() {}

    pub fn serve_order() {}

    fn take_payment() {}
}

mod test {
    fn test1() { super::serving::serve_order();}
}

pub fn house_on_fire() {
    serving::serve_order();
}
