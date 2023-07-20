use tide::prelude::*;
use tide::Body;
use tide::Request;
use tide::Response;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/status").get(status);
    app.at("/api/whoami").get(whoami);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    ip: String,
    lang: String,
    soft: String,
}

async fn whoami(req: Request<()>) -> tide::Result {
    let new_person = Person {
        ip: req.host().unwrap().to_string(),
        lang: req.header("accept-language").unwrap().to_string(),
        soft: req.header("user-agent").unwrap().to_string(),
    };

    Ok(Response::builder(200)
        .body(Body::from_json(&new_person)?)
        .build())
}

async fn status(req: Request<()>) -> tide::Result {
    Ok("server is running".into())
}
