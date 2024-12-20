use miniserve::{http::StatusCode, Content, Request, Response};
use serde::{Deserialize, Serialize};

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(Serialize, Deserialize)]
struct Messages {
    messages: Vec<String>,
}

async fn chat(req: Request) -> Response {
    match req {
        Request::Get => Ok(Content::Json(
            serde_json::to_string(&Messages { messages: vec![] }).unwrap(),
        )),
        Request::Post(body) => {
            let Ok(mut messages) = serde_json::from_str::<Messages>(&body) else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            };

            messages.messages.push("Bazinga!".into());

            Ok(Content::Json(serde_json::to_string(&messages).unwrap()))
        }
    }
}

#[tokio::main]
async fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
        .await
}
