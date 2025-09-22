struct Dummy {
    value: i32,
    name: String,
}        

fn update_dummy(d: &mut Dummy) {
    d.value += 1;
    d.name.push_str(" Updated");
}

fn main() {
    let mut dummy = Dummy {
        value: 42,
        name: String::from("Test"),
    };
    update_dummy(&mut dummy);
    println!("Hello, {} {}!", dummy.name, dummy.value);
}
