use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Id {
    pub name: Option<String>,
    pub value: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct DateOfBirth {
    pub date: Option<String>,
    pub age: Option<i64>,
}
#[derive(Debug, Deserialize)]
pub struct Login {
    pub uuid: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Timezone {
    pub offset: Option<String>,
    pub description: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Street {
    pub number: Option<i64>,
    pub name: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Location {
    pub street: Option<Street>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postcode: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub timezone: Option<Timezone>,
}
#[derive(Debug, Deserialize)]
pub struct Name {
    pub title: String,
    pub first: String,
    pub last: String,
}
#[derive(Debug, Deserialize)]
pub struct User {
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
#[derive(Debug, Deserialize)]
pub struct Users {
    pub results: Vec<User>,
}
