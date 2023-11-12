use serde::Deserialize;


struct Info {
    #[serde(skip_deserializing)]
    pub seed: String,
    #[serde(skip_deserializing)]
    pub results: i64,
    #[serde(skip_deserializing)]
    pub page: i64,
    #[serde(skip_deserializing)]
    pub version: String,
}

#[derive(Debug, Deserialize)]
struct Id {
    pub name: Option<String>,
    pub value: Option<String>,
}
#[derive(Debug, Deserialize)]
struct DateOfBirth {
    pub date: Option<String>,
    pub age: Option<i64>,
}
#[derive(Debug, Deserialize)]
struct Login {
    pub uuid: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Timezone {
    pub offset: Option<String>,
    pub description: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Coordinates {
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Street {
    pub number: Option<i64>,
    pub name: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Location {
    pub street: Option<Street>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postcode: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub timezone: Option<Timezone>,
}
#[derive(Debug, Deserialize, Default)]
struct Name {
    pub title: String,
    pub first: String,
    pub last: String,
}
#[derive(Debug, Deserialize)]
struct User {
    pub gender: Option<String>,
    pub name: Name,
    pub location: Option<Location>,
    pub email: Option<String>,
    pub login: Option<Login>,
    pub dob: Option<DateOfBirth>,
    pub registered: Option<DateOfBirth>,
    pub phone: Option<String>,
    pub cell: Option<String>,
    pub id: Option<Id>,
    pub nat: Option<String>,
}
#[derive(Debug, Deserialize, Default)]
struct Users {
    pub results: Vec<User>,
    #[serde(skip_deserializing)]
    info: Info,
}
