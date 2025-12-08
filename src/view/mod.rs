use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

pub mod about;
mod icons;
mod navbar;

use crate::config::Server;

#[get("/")]
pub async fn index_route(_server: web::Data<Server>, _req: HttpRequest) -> AwResult<Markup> {
    Ok(index(Some(about::posts())))
}

#[get("/about")]
pub async fn about_readme_endpoint() -> AwResult<Markup> {
    Ok(index(Some(about::readme())))
}

pub fn css(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {link href=(path) rel="stylesheet" type="text/css";}
}

pub fn js(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {script src=(path) {}}
}

pub fn index(content: Option<Markup>) -> Markup {
    let content = content.unwrap_or_else(about::readme);
    html! {
        (maud::DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Focus"
            }
            (js("/assets/tw.js"))
            (js("/assets/theme-switcher.js"))
            (js("/assets/htmx.js"))
            (css("/assets/daisy.css"))
            (css("/assets/themes.css"))
            (css("/assets/app.css"))
            link rel="icon" href="/assets/grocy.svg" sizes="any" type="image/svg+xml" {}

        }
        body hx-boost="true" {
            (js("/assets/htmxListener.js"))
            (js("/assets/htmx-reload.js"))


            div class="min-h-screen bg-base-100" {
                (navbar::render())
                main class="container mx-auto px-4 py-6" {
                    (content)
                }
            }
        }
    }
}
