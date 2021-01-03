use scraper::{Html};
use reqwest::get;

pub async fn get_html(url: &str) -> Html {

    let response = get(url)
        .await.unwrap()
        .text().await;

    let raw_html = response.unwrap();

    Html::parse_fragment(&raw_html)
}