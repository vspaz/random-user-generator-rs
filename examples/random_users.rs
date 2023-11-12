use random_usergenerator_rs::user::RandomUserData;

fn main() {
    let random_data = RandomUserData::new("foobar", 1, false, false);
    let users = random_data.generate();
    println!("{}", users[0].name.first);
}
