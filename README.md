
# 🌐 Crypto News Aggregator

Crypto News Aggregator — это веб-приложение, которое позволяет получать актуальные новости и цены криптовалют с различных внешних API.  
Проект включает в себя Rust-бэкенд на `warp` и фронтенд на HTML + JavaScript.

---

## 📋 Описание

Проект позволяет:

- 🔍 Ввести название/символ криптовалюты (например, BTC, ETH, SOL)
- 💬 Получить:
  - 💰 Актуальную цену и изменение за 24 часа
  - 📰 Последние новости из **нескольких внешних источников**
- 📎 Упорядоченное отображение заголовков, ссылок, источников и даты

---

## 🛠️ Установка

### Клонировать репозиторий

```bash
git clone https://github.com/your-username/crypto-news-aggregator.git
cd crypto-news-aggregator
Запустить сервер
Убедитесь, что у вас установлен Rust:

cargo run
По умолчанию сервер будет доступен на http://localhost:3030.

Открыть фронтенд
Открой файл index.html в браузере (или подключи к бэкенду как статический ресурс).

🖼️ Скриншоты
🔍 Поиск по ETH
📈 Получение цены и новостей
![🔍 Поиск по ETH 
📈 Получение цены и новостей](./screenshots/cryptonews.png)
[Результат](./screenshots/news.png)


🧪 Примеры
Пример запроса:

GET /news?query=BTC
Пример ответа (сокращённо):

json

{
  "price_info": {
    "price": 82515.01,
    "percent_change_24h": -0.3
  },
  "news": [
    {
      "title": "Bitcoin rally continues as whales accumulate",
      "url": "https://example.com/article",
      "source": "CryptoDaily",
      "date": "2025-04-06T10:00:00Z"
    }
  ]
}
