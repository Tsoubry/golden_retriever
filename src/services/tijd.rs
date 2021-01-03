use scraper::{Selector, ElementRef};
use regex::Regex;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use deadpool_postgres::Pool;
use std::time::SystemTime;

use log::info;
use crate::models::{ArticleInfo, Article};
use crate::db::add_article;
use crate::retrieve::get_html;
use std::collections::HashSet;

pub const TIJD_PLATFORM: &'static str = "De Tijd";
pub const TIJD_SECTION: &'static str = "Homepage";
pub const TIJD_URL: &'static str = "https://www.tijd.be";


pub async fn insert_all_articles(existing_articles: HashSet<String>, pool: Pool, platform: String, section: String) {

    let fragment = get_html(TIJD_URL).await;
    let article_selector = Selector::parse("div.c-articleteaser").unwrap();
    let headline_selector = Selector::parse("div.c-toparticle").unwrap();

    let title_selector = Selector::parse("div.c-articleteaser__title").unwrap();
    let headline_title_selector = Selector::parse("div.c-toparticle__title").unwrap();
    let url_selector = Selector::parse("a").unwrap();
    let image_selector = Selector::parse("img").unwrap();

    let top_article = fragment.select(&headline_selector).next();

    match top_article {
        Some(top_article) => {
            let headline_article = get_article_info(&headline_title_selector, &url_selector, &image_selector, &top_article).await;

            if !(existing_articles.contains(&headline_article.article_id)) {
                log_article(&headline_article).await;
                add_article(&pool, article_creator(headline_article, &platform, &section).await).await.unwrap_or(0);
            }
        }
        _ => {}
    };
    info!("total articles: {:?}", fragment.select(&article_selector).count());
    for element in fragment.select(&article_selector) {
        let article_info = get_article_info(&title_selector, &url_selector, &image_selector, &element).await;
        if !(existing_articles.contains(&article_info.article_id)) {
            log_article(&article_info).await;
            add_article(&pool, article_creator(article_info, &platform, &section).await).await.unwrap_or(0);
        }
    };
}

async fn get_article_info<'a>(title_selector: &Selector, url_selector: &Selector, image_selector: &Selector, element: &ElementRef<'a>)
  -> ArticleInfo {

    let title = element
        .select(title_selector)
        .next()
        .map_or("".to_string(), |m| m.text().collect::<String>())
        .replace("\n", "")
        .trim()
        .to_string()
        ;

    let article_data = element.select(url_selector).next().unwrap();

    let url = article_data.value().attr("href").unwrap_or("");
    let prefix = if url.contains(TIJD_URL) | url.contains("https://multimedia.tijd.be") { "" } else { TIJD_URL };
    let article_url = format!("{}{}", prefix, url.trim());

    let mut hasher = DefaultHasher::new();
    hasher.write(&title.as_bytes());
    let backup_id = hasher.finish().to_string();

    let article_id_pattern = Regex::new(r"/(\d+)\.html$").unwrap();
    let article_id = article_id_pattern
        .captures(&url)
        .map_or(backup_id.to_string(), |capt| {
            capt.get(1).map_or(backup_id, |m| m.as_str().to_owned())
        });

    let image_data = element.select(image_selector).next();

    let image_url = match image_data {
        Some(data) => {
            let image_blob = data.value().attr("data-srcset")
                .unwrap_or(data.value().attr("src").unwrap_or(""));
            let text_split = image_blob.split(",");
            let replaced = text_split.last().unwrap_or("").replace("&amp;", "&");
            let pattern_cleanup = Regex::new(r"\s\d+w$").unwrap();
            Some(pattern_cleanup.replace(&replaced, "").trim().to_string())
        },
        None => None,
    };

    ArticleInfo {
        article_id,
        article_title: title,
        image_url,
        article_url: Some(article_url),
    }
}

async fn article_creator(article_info: ArticleInfo, platform: &str, section: &str) -> Article {
    Article {
        id: 0,
        article_id: article_info.article_id,
        article_title: article_info.article_title,
        platform: platform.to_string(),
        section: section.to_string(),
        image_url: article_info.image_url,
        article_url: article_info.article_url,
        updated: SystemTime::now()
    }
}

async fn log_article(article_info: &ArticleInfo) {
    info!("article_id: {}, article_title: {}",article_info.article_id, article_info.article_title);
}


