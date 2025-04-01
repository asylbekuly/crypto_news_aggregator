use warp::Filter;
use std::convert::Infallible;
use serde::Deserialize;

#[derive(Clone, Debug)]
struct NewsArticle {
    title: String,
    source_name: String,
    date: String,
    text: String,
    url: String,
}

// Заглушки новостей
fn mock_news(query: &str) -> Vec<NewsArticle> {
    vec![
        NewsArticle {
            title: format!("{} взлетает до луны!", query),
            source_name: "CryptoBlog".into(),
            date: "2025-04-01".into(),
            text: format!("{} показывает резкий рост за последние 24 часа...", query),
            url: "https://cryptoblog.com/fake-news1".into(),
        },
        NewsArticle {
            title: format!("Инвесторы интересуются {}", query),
            source_name: "CoinTimes".into(),
            date: "2025-03-30".into(),
            text: format!("Все больше аналитиков рекомендуют покупать {}.", query),
            url: "https://cointimes.com/fake-news2".into(),
        },
    ]
}

// HTML-шаблон страницы
fn render_page(coin: Option<String>, articles: Vec<NewsArticle>) -> String {
    let mut html = String::new();

    html.push_str(r#"
    <!DOCTYPE html>
    <html lang="ru">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Crypto News</title>
        <script src="https://cdn.tailwindcss.com"></script>
    </head>
    <body class="bg-gray-100 text-gray-800 font-sans p-6">
        <div class="max-w-3xl mx-auto">
            <h1 class="text-3xl font-bold mb-6 text-center">📰 Crypto News Aggregator</h1>

            <form action="/" method="get" class="flex gap-2 mb-8">
                <input
                    type="text"
                    name="coin"
                    placeholder="Введите тикер, например BTC"
                    class="flex-1 px-4 py-3 border rounded-xl border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                >
                <button
                    type="submit"
                    class="bg-blue-600 text-white px-6 py-3 rounded-xl hover:bg-blue-700 transition"
                >Найти</button>
            </form>
    "#);

    if let Some(c) = coin {
        html.push_str(&format!(
            r#"<h2 class="text-xl font-semibold mb-4">Результаты для: {}</h2>"#,
            c
        ));

        for article in articles {
            html.push_str(&format!(
                r#"
                <div class="bg-white rounded-xl shadow-md p-5 mb-4">
                    <h3 class="text-lg font-bold mb-1">{}</h3>
                    <p class="text-sm text-gray-500 mb-2">{} • {}</p>
                    <p class="mb-3">{}</p>
                    <a href="{}" target="_blank" class="text-blue-600 hover:underline">Читать далее</a>
                </div>
                "#,
                article.title, article.source_name, article.date, article.text, article.url
            ));
        }
    }

    html.push_str("</div></body></html>");
    html
}


#[derive(Debug, Deserialize)]
struct Query {
    coin: Option<String>,
}

#[tokio::main]
async fn main() {
    let route = warp::path::end()
        .and(warp::query::<Query>())
        .and_then(handle_request);

    println!("🚀 Сервер запущен на http://localhost:3030");
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_request(query: Query) -> Result<impl warp::Reply, Infallible> {
    let (coin, articles) = if let Some(ref coin) = query.coin {
        (Some(coin.clone()), mock_news(coin))
    } else {
        (None, vec![])
    };

    let html = render_page(coin, articles);
    Ok(warp::reply::html(html))
}
