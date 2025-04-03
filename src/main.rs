use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};
use std::collections::HashMap;

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

#[derive(Debug, Deserialize, Serialize)]
struct CoinMarketCapResponse {
    data: HashMap<String, CoinData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CoinData {
    symbol: String,
    quote: QuoteData,
}

#[derive(Debug, Deserialize, Serialize)]
struct QuoteData {
    #[serde(rename = "USD")]
    usd: PriceInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PriceInfo {
    price: f64,
    volume_24h: f64,
    percent_change_24h: f64,
}

#[derive(Debug, Serialize)]
struct CombinedResponse {
    query: String,
    price_info: Option<PriceInfo>,
    news: Vec<NewsItem>,
}

#[tokio::main]
async fn main() {
    println!("🚀 Server running at http://localhost:3030");

    let news_route = warp::path("news")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handle_news);

    let static_files = warp::fs::file("index.html");

    let routes = news_route.or(static_files);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_news(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(query) = params.get("query") {
        let news = fetch_news(query).await.unwrap_or_default();
        let price = fetch_coin_price(query).await.ok();

        let response = CombinedResponse {
            query: query.to_string(),
            price_info: price,
            news,
        };

        Ok(warp::reply::json(&response).into_response())
    } else {
        Ok(warp::reply::with_status(
            "Missing query",
            warp::http::StatusCode::BAD_REQUEST,
        )
        .into_response())
    }
}

async fn fetch_news(query: &str) -> Result<Vec<NewsItem>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "https://crypto-news54.p.rapidapi.com/v2/media?orderby=date&order=desc&context=view&status=inherit&per_page=20&page=1";

    let response = client
        .get(url)
        .header("X-RapidAPI-Key", "d72c95cd6fmsh1258d9a3f329cfap167ffdjsn8554ec9e8fac")
        .header("X-RapidAPI-Host", "crypto-news54.p.rapidapi.com")
        .send()
        .await?;

    let json: ApiResponse = response.json().await?;

    let filtered_news: Vec<NewsItem> = json
        .data
        .into_iter()
        .filter(|news| {
            let query_lc = query.to_lowercase();
            news.title_obj.rendered.to_lowercase().contains(&query_lc)
                || news.description_obj.as_ref().map_or(false, |desc| {
                    desc.rendered.to_lowercase().contains(&query_lc)
                })
        })
        .collect();

    Ok(filtered_news)
}

async fn fetch_coin_price(symbol: &str) -> Result<PriceInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
        symbol
    );

    let response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", "90c60a9c-1555-4163-92b8-7c28011b95db")
        .header("Accept", "application/json")
        .send()
        .await?;

    let result: CoinMarketCapResponse = response.json().await?;

    let price_info = result
        .data
        .get(&symbol.to_uppercase())
        .map(|coin| coin.quote.usd.clone())
        .ok_or("Symbol not found in CoinMarketCap")?;

    Ok(price_info)
}
