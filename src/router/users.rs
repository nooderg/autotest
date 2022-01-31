#[post("/user")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/users/<id>")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[update("/users")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[delete("/users")]
fn hello() -> &'static str {
    "Hello, world!"
}