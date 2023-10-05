use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};


fn head(title: &str, more: Option<Markup>) -> Markup {
  html! {
    head {
      meta charset="utf-8";
      meta name="viewport" content="width=device-width, initial-scale=1" {}
      title { (title) }
      script src="https://unpkg.com/htmx.org@1.9.6" {}
      script src="https://cdn.tailwindcss.com" {}
      (more.unwrap_or_default())
    }
  }
}

fn header(active_tab: Option<&str>) -> Markup {
  let tabs = [
    ("Home", "/", active_tab == Some("home")),
    ("About", "/about", active_tab == Some("about")),
    ("Login", "/login", active_tab == Some("login")),
  ];
  html! {
    header  {
      div.flex."max-w-7xl"."mx-auto" {
        a href="/" {
          "WhatNow?"
        }
        div."flex-grow" {}
        nav {
          ul.flex."space-x-4" {
            @for (label, href, active) in tabs.iter() {
              li.{({ if *active { "bg-gray-900" } else { "bg-gray-700" } })} {
                a href=(href) {
                  (label)
                }
              }
            }
          }
        }
      }
    }
  }
}

async fn index() -> Markup {
  html! {
    (DOCTYPE)
    html lang="en" {
      (head("WhatNow", None))
      body {
        (header(Some("home")))
        h1 { "Hello, world!" }
      }
    }
  }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .route("/", get(index))
  ;
  Ok(router.into())
}
