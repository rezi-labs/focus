use actix_web::{HttpResponse, Result as AwResult, get, web};
use chrono::{DateTime, Utc};
use include_dir::{Dir, include_dir};
use lazy_static::lazy_static;
use markdown::{self, CompileOptions, Options};
use maud::{Markup, PreEscaped, html};

static POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/posts");

#[derive(Debug, Clone)]
struct Post {
    title: String,
    subtitle: String,
    md_content: String,
    date: DateTime<Utc>,
}

impl Post {
    pub fn human_date(&self) -> String {
        self.date.format("%B %d, %Y").to_string()
    }
}

fn post_to_html(post: Post, current_index: usize) -> maud::Markup {
    let as_html = markdown::to_html_with_options(
        &post.md_content,
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
                    h1 {(post.title)}
                    p {(post.subtitle)}
                    p {(post.human_date())}
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

fn parse_post_content(content: &str) -> (String, String, String) {
    let lines: Vec<&str> = content.lines().collect();
    let mut title = String::from("Untitled");
    let mut subtitle = String::new();
    let mut content_start_idx = 0;

    // Find title (first line starting with #)
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            title = trimmed.trim_start_matches('#').trim().to_string();
            content_start_idx = idx + 1;
            break;
        }
    }

    // Find subtitle (text between title and "---")
    let mut subtitle_lines = Vec::new();
    let mut found_separator = false;

    for (offset, line) in lines[content_start_idx..].iter().enumerate() {
        let trimmed = line.trim();

        if trimmed == "---" {
            found_separator = true;
            content_start_idx = content_start_idx + offset + 1;
            break;
        }

        if !trimmed.is_empty() {
            subtitle_lines.push(trimmed);
        }
    }

    if !subtitle_lines.is_empty() {
        subtitle = subtitle_lines.join(" ");
    }

    // Get content after separator
    let content = if found_separator && content_start_idx < lines.len() {
        lines[content_start_idx..].join("\n")
    } else {
        content.to_string()
    };

    (title, subtitle, content)
}

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
                let (title, subtitle, md_content) = parse_post_content(content);

                Some(Post {
                    title,
                    subtitle,
                    md_content,
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

pub fn posts() -> Markup {
    match get_post(0) {
        Some(post) => post_to_html(post, 0),
        None => html! {
            div class="space-y-6" {
                p { "No posts available yet." }
            }
        },
    }
}
