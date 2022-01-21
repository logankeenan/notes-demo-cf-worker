use std::str::FromStr;
use notes_demo::AppState;
use worker::*;
use tide::http::{Method, Url, Request as TideRequest, Response as TideResponse};

mod utils;

async fn to_tide_request(mut request: Request) -> TideRequest {
    let method = Method::from_str(request.method().to_string().as_str()).unwrap();
    let url = Url::from_str(request.url().unwrap().as_str()).unwrap();
    let mut tide_request = TideRequest::new(method, url);

    let body_text = request.text().await.unwrap();
    tide_request.set_body(body_text);

    for (key, value) in request.headers() {
        tide_request.insert_header(key.as_str(), value.as_str());
    }

    tide_request
}

async fn to_worker_response(mut tide_response: TideResponse) -> Response {
    let bytes = tide_response.body_bytes().await.unwrap();
    let response = Response::from_bytes(bytes).unwrap();
    let code: u16 = tide_response.status().to_string().parse().unwrap();
    let response = response.with_status(code);

    let mut headers = Headers::new();

    tide_response.header_names().for_each(|header_name| {
        let header_value = tide_response.header(header_name).unwrap();
        headers.set(header_name.as_str(), header_value.as_str()).unwrap();
    });
    let response = response.with_headers(headers);

    response
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(worker_request: Request, env: Env) -> Result<Response> {
    log_request(&worker_request);
    utils::set_panic_hook();

    let mut app_state = AppState::new();
    app_state.environment.insert(String::from("API_ORIGIN"), env.var("API_ORIGIN")?.to_string());
    let app = notes_demo::create(app_state);

    let app_response: TideResponse = app.respond(to_tide_request(worker_request).await).await.unwrap();
    let worker_response = to_worker_response(app_response).await;
    Ok(worker_response)
}
