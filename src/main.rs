use std::convert::Infallible;
use anyhow::Result;
use log::info;
use warp::Filter;

pub async fn check(url: String) -> Result<impl warp::Reply, Infallible> {
    let url = format!("http://{}", url);
    info!("Checking URL {}", url);
    let resp = match reqwest::get(&url).await {
        Ok(resp) => format!("Got code {:?} for {}", resp.status(), url),
        Err(e) => format!("Failed, {:?}, tried {}", e, url),
    };
    info!("{}", &resp);
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(_) = std::env::var("RUST_LOG") { std::env::set_var("RUST_LOG", "info"); }
    pretty_env_logger::init();

    let routes = warp::path!("check" / String)
        .and(warp::get())
        .and_then(check);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}
