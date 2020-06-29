#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod other;
mod form_multiple_segments;
use rocket_contrib::serve::StaticFiles;
use rocket::http::RawStr;
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hellow"
}

#[get("/hello/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello {}", &name)
}

#[get("/amIcool/<cool>")]
fn cool(cool: bool) -> &'static str {
    if cool {
        "You're so cool!"
    } else {
        "You're not cool..."
    }
}

// matches against all paths that begin with /page/
#[get("/page/<file..>")]
fn multiple_segments(file: PathBuf) -> String {
    format!("Your file path: {:?}", file)
}

#[get("/hello?<name>")] // For example: localhost:8000/hello?name=Loeka
fn hello_with_query_string(name: &RawStr) -> String {
    format!("Hello, {}", name.as_str())
}

#[get("/maybehello?<name>")]
fn maybe_hello_with_name(name: Option<String>) -> String {
    match name {
        Some(i) => format!("Hello, {}", i),
        None => String::from("Hello!")
    }
    // Same as below
    // name.map(|name| format!("Hello, {}", name))
    //     .unwrap_or_else(|| String::from("Hello!"))
}

fn main() {
    let mut static_path = PathBuf::new();
    static_path.push(std::env::current_dir().unwrap());
    static_path.push("static");
    println!("PATH: {:?}", static_path);
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                hello,
                other::func_from_other_file,
                hello_name,
                cool,
                multiple_segments,
                hello_with_query_string,
                maybe_hello_with_name,
                form_multiple_segments::item
            ],
        )
        .mount("/website", StaticFiles::from(static_path))
        .launch();

}
