use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use warp::{Filter, Reply};

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

#[derive(Debug, Deserialize, Serialize)]
struct DailyNewsItem {
    title: String,
    url: String,
    source: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DailyResponse {
    data: Vec<DailyNewsItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExtraNewsItem {
    title: String,
    url: String,
    published_at: String,
}

#[derive(Debug, Deserialize)]
struct ExtraResponse {
    data: Vec<ExtraNewsItem>,
}

#[derive(Debug, Serialize)]
struct UnifiedArticle {
    title: String,
    url: String,
    source: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
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
        let mut news = vec![];

        if let Ok(daily) = fetch_daily_news().await {
            for a in daily {
                if a.title.to_lowercase().contains(&query.to_lowercase()) {
                    news.push(UnifiedArticle {
                        title: a.title,
                        url: a.url,
                        source: a.source.unwrap_or("Daily".to_string()),
                    });
                }
            }
        }

        if let Ok(extra) = fetch_extra_news().await {
            for a in extra {
                if a.title.to_lowercase().contains(&query.to_lowercase()) {
                    news.push(UnifiedArticle {
                        title: a.title,
                        url: a.url,
                        source: "Extra".to_string(),
                    });
                }
            }
        }

        let price = fetch_coin_price(query).await.ok();

        let response = warp::reply::json(&serde_json::json!({
            "query": query,
            "price_info": price,
            "news": news
        }));

        return Ok(response);
    }

    Ok(warp::reply::json(&serde_json::json!({ "error": "Missing query" })))
    
}

async fn fetch_coin_price(symbol: &str) -> Result<PriceInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let cmc_api_key = env::var("CMC_API_KEY")?;
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
        symbol
    );

    let response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", cmc_api_key)
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

async fn fetch_daily_news() -> Result<Vec<DailyNewsItem>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let key = env::var("RAPIDAPI_KEY")?;

    let response = client
        .get("https://cryptocurrency-news2.p.rapidapi.com/v1/cryptodaily")
        .header("x-rapidapi-key", &key)
        .header("x-rapidapi-host", "cryptocurrency-news2.p.rapidapi.com")
        .send()
        .await?;

    let json: DailyResponse = response.json().await?;
    Ok(json.data)
}

async fn fetch_extra_news() -> Result<Vec<ExtraNewsItem>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let key = env::var("RAPIDAPI_KEY")?;

    let response = client
        .get("https://news-api65.p.rapidapi.com/api/v1/crypto/articles/search?format=json&time_frame=24h&page=1&limit=10")
        .header("x-rapidapi-key", &key)
        .header("x-rapidapi-host", "news-api65.p.rapidapi.com")
        .send()
        .await?;

    let json: ExtraResponse = response.json().await?;
    Ok(json.data)
}
