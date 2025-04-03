use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};

#[derive(Debug, Deserialize, Serialize)]
struct RenderedField {
    rendered: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NewsItem {
    id: u64,
    date: String,
    link: String,

    #[serde(rename = "title")]
    title_obj: RenderedField,

    #[serde(rename = "description")]
    description_obj: Option<RenderedField>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<NewsItem>,
}

#[tokio::main]
async fn main() {
    let news_route = warp::path("news")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(handle_news);

    println!("🚀 Server running at http://localhost:3030");
    let static_files = warp::fs::file("index.html");

    let routes = warp::path("news")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(handle_news)
        .or(static_files);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_news(
    params: std::collections::HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(query) = params.get("query") {
        match fetch_news(query).await {
            Ok(news) => Ok(warp::reply::json(&news).into_response()),
            Err(e) => {
                eprintln!("❌ Error fetching news: {}", e);
                Ok(warp::reply::with_status(
                    "Error fetching news",
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response())
            }
        }
    } else {
        Ok(
            warp::reply::with_status("Missing query", warp::http::StatusCode::BAD_REQUEST)
                .into_response(),
        )
    }
}

async fn fetch_news(_query: &str) -> Result<Vec<NewsItem>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "https://crypto-news54.p.rapidapi.com/v2/media?orderby=date&order=desc&context=view&status=inherit&per_page=20&page=1";

    let response = client
        .get(url)
        .header(
            "X-RapidAPI-Key",
            "d72c95cd6fmsh1258d9a3f329cfap167ffdjsn8554ec9e8fac",
        )
        .header("X-RapidAPI-Host", "crypto-news54.p.rapidapi.com")
        .send()
        .await?;

    let json: ApiResponse = response.json().await?;

    // 🟢 Просто вернём все данные без фильтра
    Ok(json.data)
}
