struct Dummy {
    value: i32,
    name: String,
}        

fn update_value(d: &mut Dummy) {
    d.value += 100;
}

fn update_name(d: &mut Dummy) {
    d.name = d.name.to_uppercase();
    d.name.push_str(".upname.");
}

fn update_dummy(d: &mut Dummy) {
    d.value += 10;
    update_value(d);
    update_name(d);
    d.name.push_str(" Updated");
    d.value += 1;
}

fn main() {
    let mut dummy = Dummy {
        value: 1,
        name: String::from("Main"),
    };
    update_dummy(&mut dummy);
    println!("Name: {}, Value: {}", dummy.name, dummy.value);
}
