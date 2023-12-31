use random_data_generator::user::Data;

fn main() {
    let random_data = Data::new("foobar", 10, true, false);
    let users = random_data.generate().unwrap();
    let user = &users[9].name;
    println!("{}. {} {}", user.title, user.first, user.last);
}
