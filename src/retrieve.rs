use scraper::{Html};
use isahc::{cookies::CookieJar, prelude::*, Request};

pub async fn get_html(url: &str) -> Html {

    let cookie_jar = CookieJar::new();

    let mut response = Request::get(url)
        // Set the cookie jar to use for this request.
        .cookie_jar(cookie_jar.clone())
        .body(()).unwrap()
        .send().unwrap();

    let raw_html = response.text().unwrap();

    Html::parse_fragment(&raw_html)
}