use rocket::request::Form;

#[derive(FromForm, Debug)]
pub struct User {
    name: String,
    account: usize
}

#[get("/item?<id>&<user..>")] // For example localhost:8000/item?id=123&name=myitem&account=10
pub fn item(id: usize, user: Option<Form<User>>) -> String {
    match user {
        Some(i) => format!("Item {:?} with id: {}", i, id),
        None => format!("Item didn't had a name and/or account.")
    }
}