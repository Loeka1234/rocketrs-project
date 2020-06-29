#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod other;
use rocket_contrib::serve::StaticFiles;
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
                multiple_segments
            ],
        )
        .mount("/website", StaticFiles::from(static_path))
        .launch();

}
