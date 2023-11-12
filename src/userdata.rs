use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
struct Info {
    pub seed: String,
    pub results: i64,
    pub page: i64,
    pub version: String,
}

#[derive(Debug, Deserialize, Default)]
struct Id {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Deserialize, Default)]
struct DateOfBirth {
    pub date: String,
    pub age: i64,
}
#[derive(Debug, Deserialize, Default)]
struct Login {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
}
#[derive(Debug, Deserialize, Default)]
struct Timezone {
    pub offset: String,
    pub description: String,
}
#[derive(Debug, Deserialize, Default)]
struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}
#[derive(Debug, Deserialize, Default)]
struct Street {
    pub number: i64,
    pub name: String,
}
#[derive(Debug, Deserialize, Default)]
struct Location {
    pub street: Street,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postcode: String,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}
#[derive(Debug, Deserialize, Default)]
struct Name {
    pub title: String,
    pub first: String,
    pub last: String,
}
#[derive(Debug, Deserialize, Default)]
struct User {
    pub gender: String,
    pub name: Name,
    pub location: Location,
    pub email: String,
    pub login: Login,
    pub dob: DateOfBirth,
    pub registered: DateOfBirth,
    pub phone: String,
    pub cell: String,
    pub id: Id,
    pub nat: String,
}
#[derive(Debug, Deserialize, Default)]
struct Users {
    pub results: Vec<User>,
    info: Info,
}
