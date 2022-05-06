use notes_demo::AppState;
use worker::*;
use tide::http::{Response as TideResponse};
use rora_tide_adapter::cf_worker;

mod utils;

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

    let app_response: TideResponse = app.respond(cf_worker::to_tide_request(worker_request).await).await.unwrap();
    let worker_response = cf_worker::to_response(app_response).await;
    Ok(worker_response)
}
