#[macro_use] extern crate rocket;

mod tera;

#[cfg(test)] mod tests;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        //.mount("/", routes![index])
        .mount("/", routes![tera::index, tera::hello, tera::about])
        .register("/", catchers![tera::not_found])
        .attach(Template::custom(|engines| {
            tera::customize(&mut engines.tera);
        }))
}