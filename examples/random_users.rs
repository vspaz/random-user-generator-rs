use random_usergenerator_rs::user::RandomUserData;

fn main() {
    let random_data = RandomUserData::new("foobar", 10, true, false);
    let users = random_data.generate().unwrap();
    println!("{}", users[0].name.first);
}
