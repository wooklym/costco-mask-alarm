use std::env;
use std::thread::sleep;
use std::time::Duration;

use dotenv::dotenv;
use scraper::{Html, Selector};
use tgbot::{
    methods::SendMessage,
    types::{ChatId, Integer},
    Api, Config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("Failed to load .env file");

    let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID is not set");
    let chat_id = match chat_id.parse::<Integer>() {
        Ok(chat_id) => ChatId::Id(chat_id),
        Err(_) => ChatId::Username(chat_id),
    };
    let config = Config::new(token);
    let api = Api::new(config).expect("Failed to create API");

    loop {
        let resp = reqwest::get(
            "https://www.costco.co.kr/HealthSupplement/Home-Health-CareFirst-Aid/First-Aid/c/cos_12.7.2",
        )
        .await?
        .text()
        .await?;

        let document = Html::parse_document(&resp);

        let selector = Selector::parse(r#"div[class="product-name-container"]"#).unwrap();

        for el in document.select(&selector) {
            let texts = el.text().collect::<Vec<_>>();

            let name = texts.get(1).unwrap().to_owned();
            let link = el.select(&Selector::parse("a").unwrap()).next().unwrap();

            if name.contains("kf") || name.contains("KF") {
                let msg = format!(
                    "{} - {}{}",
                    name,
                    "https://www.costco.co.kr",
                    link.value().attr("href").unwrap_or("cannot get a link")
                );

                api.execute(SendMessage::new(chat_id.clone(), msg))
                    .await
                    .unwrap();
            }
        }

        sleep(Duration::from_secs(60));
    }
}
