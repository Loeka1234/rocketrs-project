#[get("/other")]
pub fn func_from_other_file() -> &'static str {
    "Hello from another file!"
}