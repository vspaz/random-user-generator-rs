
struct Info {
    pub seed: String,
    pub results: i64,
    pub page: i64,
    pub version: String,
}

struct Id {
    pub name: String,
    pub value: String,
}

struct DateOfBirth {
    pub date: String,
    pub age: i64,
}

struct Login {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
}

struct Timezone {
    pub offset: String,
    pub description: String,
}

struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

struct Street {
    pub number: i64,
    pub name: String,
}

struct Location {
    pub street: Street,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postcode: String,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}

struct Name {
    pub title: String,
    pub first: String,
    pub last: String,
}

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

struct Users {
    pub results: Vec<User>,
    info: Info,
}