use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;
use shuttle_metadata::Metadata;
use yaml_front_matter::{Document, YamlFrontMatter};

async fn hello_world(State(state): State<AppState>) -> impl IntoResponse {
    let markdown_input = match tokio::fs::read("templates/index.md").await {
        Ok(res) => res,
        Err(_) => return Html("Couldn't find this page - does it exist?".to_string()),
    };

    let string_output = std::str::from_utf8(&markdown_input).unwrap();

    let mut html_output = get_page_header(string_output, &state.domain);

    let regex = regex::Regex::new(r"---((.|\n)*?)---").unwrap();

    let res = regex.replace(string_output, "");
    let parser = pulldown_cmark::Parser::new(&res);
	
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output.push_str("<div id=\"content\">");
    for link in &state.filenames {
	html_output.push_str(&format!("<a href=\"/{link}\">{link}</a>"));
	}
    html_output.push_str("<div id=\"content\">");

    Html(html_output)
}

#[derive(Clone)]
struct AppState {
    domain: String,
    filenames: Vec<String>
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_metadata::ShuttleMetadata] metadata: Metadata,
) -> shuttle_axum::ShuttleAxum {
    let domain = if cfg!(debug_assertions) {
        "http://localhost:8000".to_string()
    } else {
        format!("https://{}.shuttleapp.rs", metadata.project_name)
    };

    let mut files = tokio::fs::read_dir("templates").await.unwrap();

    let mut filenames: Vec<String> = Vec::new();

    while let Some(file) = files.next_entry().await.unwrap() {
		let filename = file.file_name().into_string().unwrap();

	   if filename.ends_with(".md") &&  filename !=  *"index.md" {
		filenames.push(filename.replace(".md", ""));
		}
	}

    let state = AppState { domain , filenames};

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/:page", get(parser))
        .route("/:dir/:page", get(parser_dir))
        .route("/styles.css", get(styles))
        .with_state(state);

    Ok(router.into())
}

async fn parser(State(state): State<AppState>, Path(page): Path<String>) -> impl IntoResponse {
    let mut file = format!("templates/{page}.md");

    let markdown_input = match tokio::fs::read(file).await {
        Ok(res) => res,
        Err(_) => {
            file = format!("templates/{page}/index.md");
            match tokio::fs::read(file).await {
                Ok(res) => res,
                Err(_) => return Html("Couldn't find this page - does it exist?".to_string()),
            }
        }
    };

    let string_output = std::str::from_utf8(&markdown_input).unwrap();

    let mut html_output = get_page_header(string_output, &format!("{}/{page}", &state.domain));

    let regex = regex::Regex::new(r"---((.|\n)*?)---").unwrap();

    let res = regex.replace(string_output, "");
    let parser = pulldown_cmark::Parser::new(&res);
    pulldown_cmark::html::push_html(&mut html_output, parser);

    Html(html_output)
}

async fn parser_dir(
    State(state): State<AppState>,
    Path((dir, page)): Path<(String, String)>,
) -> impl IntoResponse {
    let file = format!("templates/{dir}/{page}.md");

    let markdown_input = match tokio::fs::read(file).await {
        Ok(res) => res,
        Err(_) => return Html("Couldn't find this page - does it exist?".to_string()),
    };

    let string_output = std::str::from_utf8(&markdown_input).unwrap();

    let mut html_output = get_page_header(string_output, &format!("{}/{dir}/{page}", state.domain));

    let regex = regex::Regex::new(r"---((.|\n)*?)---").unwrap();

    let res = regex.replace(string_output, "");
    let parser = pulldown_cmark::Parser::new(&res);
    pulldown_cmark::html::push_html(&mut html_output, parser);

    Html(html_output)
}

async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())
        .unwrap()
}

#[derive(Deserialize)]
struct PageMetadata {
    title: String,
    description: String,
}

fn get_page_header(input: &str, domain: &str) -> String {
    let document: Document<PageMetadata> = YamlFrontMatter::parse::<PageMetadata>(input).unwrap();
    let PageMetadata { title, description } = document.metadata;

    let mut html_output = String::new();
    html_output.push_str("<head>");
    html_output.push_str(
        r#"<meta charset="UTF-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <meta http-equiv="X-UA-Compatible" content="ie=edge"/>
    <link rel="stylesheet" type="text/css" href="/styles.css">"#);
    html_output.push_str(&format!(
        "<title>{title}</title>
		<meta property=\"og:title\" content=\"{title}\"/>
		<meta property=\"og:url\" content=\"{domain}\"/>
		<meta property=\"og:description\" content=\"{description}\"/>"
    ));
    html_output.push_str("</head>");

    html_output.push_str("<div id=\"nav\">");
	html_output.push_str("<a href=\"/\">Home</a>");
    html_output.push_str("</div>");
    html_output
}
