use actix_web::{HttpResponse, Result as AwResult, get, web};
use chrono::{DateTime, Utc};
use include_dir::{Dir, include_dir};
use lazy_static::lazy_static;
use markdown::{self, CompileOptions, Options};
use maud::{Markup, PreEscaped, html};

static POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/posts");

#[derive(Debug, Clone)]
struct Post {
    slug: String,
    content: String,
    date: DateTime<Utc>,
}

impl Post {
    pub fn human_date(&self) -> String {
        self.date.format("%B %d, %Y").to_string()
    }
}

fn post_to_html(post: Post, current_index: usize) -> maud::Markup {
    let as_html = markdown::to_html_with_options(
        &post.content,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: false,
                ..CompileOptions::default()
            },
            ..Options::gfm()
        },
    )
    .unwrap();

    let next_index = current_index + 1;
    let next_url = format!("/posts/{next_index}");
    let has_next = next_index < POSTS.len();
    let has_prev = current_index > 0;

    maud::html! {
        div id="post" class="flex flex-col h-full" {
            div class="flex-1 overflow-y-auto space-y-6 pb-6" {
                div class="prose" {
                    p class="post-date" {(post.human_date())}
                }

                div class="divider" {}
                (PreEscaped(as_html))
            }

            div class="mt-6 flex justify-between sticky bottom-0 bg-base-100 py-4 border-t border-base-200" {
                @if has_prev {
                    button
                        class="btn btn-primary"
                        hx-get={"/posts/" (current_index - 1)}
                        hx-target="#post"
                        hx-swap="outerHTML" {
                        "Previous Post"
                    }
                } @else {
                    div {}
                }
                @if has_next {
                    button
                        class="btn btn-primary"
                        hx-get=(next_url)
                        hx-target="#post"
                        hx-swap="outerHTML" {
                        "Next Post"
                    }
                }
            }
        }
    }
}

fn post_to_html_with_slug(post: Post, current_index: usize) -> maud::Markup {
    let as_html = markdown::to_html_with_options(
        &post.content,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: false,
                ..CompileOptions::default()
            },
            ..Options::gfm()
        },
    )
    .unwrap();

    let (prev_slug, next_slug) = get_adjacent_post_slugs(current_index);

    maud::html! {
        div id="post" class="flex flex-col h-full" {
            div class="flex-1 overflow-y-auto space-y-6 pb-6" {

                (PreEscaped(as_html))
            }

            div class="mt-6 flex justify-between sticky bottom-0 bg-base-100 py-4 border-t border-base-200" {
                @if let Some(prev) = prev_slug {
                    button
                        class="btn btn-primary"
                        hx-get={"/post/" (prev)}
                        hx-target="#post"
                        hx-swap="outerHTML"
                        hx-push-url="true" {
                        "Previous Post"
                    }
                } @else {
                    div {}
                }
                div {
                    p class="post-date" {(post.human_date())}
                }
                @if let Some(next) = next_slug {
                    button
                        class="btn btn-primary"
                        hx-get={"/post/" (next)}
                        hx-target="#post"
                        hx-swap="outerHTML"
                        hx-push-url="true" {
                        "Next Post"
                    }
                } @else {
                    div {}
                }
            }
        }
    }
}

fn extract_slug_from_filename(filename: &str) -> String {
    // Parse slug from filename (format: YYYY-MM-DD-slug.md)
    if filename.len() > 11 {
        // Skip the date part (YYYY-MM-DD-) and get the slug
        let slug_part = &filename[11..];
        slug_part.to_string()
    } else {
        // Fallback: generate slug from filename
        filename
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c
                } else if c.is_whitespace() {
                    '-'
                } else {
                    '\0'
                }
            })
            .filter(|&c| c != '\0')
            .collect::<String>()
    }
}

use std::collections::HashMap;

lazy_static! {
    static ref POSTS: Vec<Post> = {
        let mut posts: Vec<Post> = POSTS_DIR
            .files()
            .filter_map(|file| {
                let path = file.path();

                // Only process markdown files
                if path.extension().is_none_or(|ext| ext != "md") {
                    return None;
                }

                // Parse date from filename (format: YYYY-MM-DD-title.md)
                let filename = path.file_stem()?.to_str()?;
                let date = if filename.len() >= 10 {
                    let date_str = &filename[..10];
                    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                        .ok()
                        .and_then(|naive_date| {
                            naive_date.and_hms_opt(0, 0, 0)
                                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                        })
                        .unwrap_or_else(Utc::now)
                } else {
                    Utc::now()
                };

                let content = file.contents_utf8().unwrap_or("");
                let slug = extract_slug_from_filename(filename);

                Some(Post {
                    slug,
                    content: content.to_string(),
                    date,
                })
            })
            .collect();

        // Sort by date (newest first)
        posts.sort_by(|a, b| b.date.cmp(&a.date));
        posts
    };
}

lazy_static! {
    static ref SLUG_TO_INDEX: HashMap<String, usize> = {
        let mut map = HashMap::new();
        for (index, post) in POSTS.iter().enumerate() {
            map.insert(post.slug.clone(), index);
        }
        map
    };
}

lazy_static! {
    static ref README_HTML: String = {
        let markdown_content = include_str!("../../README.md");
        let html_output = markdown::to_html_with_options(
            markdown_content,
            &Options {
                compile: CompileOptions {
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: false,
                    ..CompileOptions::default()
                },
                ..Options::gfm()
            },
        )
        .unwrap();

        format!("<div class=\"space-y-6\">{html_output}</div>")
    };
}

pub fn readme() -> Markup {
    html! {
         (PreEscaped(&*README_HTML))
    }
}

fn get_post(page: usize) -> Option<Post> {
    POSTS.get(page).cloned()
}

fn get_post_by_slug(slug: &str) -> Option<(Post, usize)> {
    SLUG_TO_INDEX
        .get(slug)
        .and_then(|&index| POSTS.get(index).map(|post| (post.clone(), index)))
}

fn get_adjacent_post_slugs(current_index: usize) -> (Option<String>, Option<String>) {
    let prev_slug = if current_index > 0 {
        POSTS.get(current_index - 1).map(|post| post.slug.clone())
    } else {
        None
    };

    let next_slug = POSTS.get(current_index + 1).map(|post| post.slug.clone());

    (prev_slug, next_slug)
}

#[get("/posts/{index}")]
pub async fn post_route(path: web::Path<usize>) -> AwResult<HttpResponse> {
    let index = path.into_inner();

    match get_post(index) {
        Some(post) => {
            let html = post_to_html(post, index);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(html.into_string()))
        }
        None => Ok(HttpResponse::NotFound().body("No more posts")),
    }
}

#[get("/post/{slug}")]
pub async fn post_slug_route(path: web::Path<String>) -> AwResult<HttpResponse> {
    let slug = path.into_inner();

    match get_post_by_slug(&slug) {
        Some((post, index)) => {
            let html = post_to_html_with_slug(post, index);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(html.into_string()))
        }
        None => Ok(HttpResponse::NotFound().body("Post not found")),
    }
}

pub fn posts() -> Markup {
    match get_post(0) {
        Some(post) => post_to_html_with_slug(post, 0),
        None => html! {
            div class="space-y-6" {
                p { "No posts available yet." }
            }
        },
    }
}
