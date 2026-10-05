use crate::utils::client::create_client;
use reqwest::header::CONTENT_TYPE;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;

pub async fn crawl(
    start_url: &Url,
    target_host: &str,
    root_domain: &str,
) -> Result<(HashSet<String>, HashSet<String>), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let mut visited = HashSet::new();
    let mut to_visit = vec![start_url.clone()];

    let mut target_urls = HashSet::new();
    let mut subdomain_urls = HashSet::new();

    let a_selector = Selector::parse("a[href]").unwrap();

    // 簡易クローラー (安全のため最大100ページに制限)
    while let Some(current_url) = to_visit.pop() {
        let current_str = current_url.to_string();
        if visited.contains(&current_str) || visited.len() >= 100 {
            continue;
        }
        visited.insert(current_str);

        let response = match client.get(current_url.clone()).send().await {
            Ok(res) => res,
            Err(_) => continue,
        };

        // HTMLコンテンツのみをパース
        if let Some(ct) = response.headers().get(CONTENT_TYPE) {
            if !ct.to_str().unwrap_or("").contains("text/html") {
                continue;
            }
        } else {
            continue;
        }

        let body = match response.text().await {
            Ok(b) => b,
            Err(_) => continue,
        };

        let document = Html::parse_document(&body);

        for element in document.select(&a_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(parsed_link) = current_url.join(href) {
                    // フラグメント(#)の除外
                    let mut clean_link = parsed_link.clone();
                    clean_link.set_fragment(None);
                    let clean_str = clean_link.to_string();

                    if let Some(link_host) = clean_link.host_str() {
                        // ルートドメイン不一致の外部リンクは除外
                        if !link_host.ends_with(root_domain) {
                            continue;
                        }

                        // 同一ホスト (完全一致)
                        if link_host == target_host {
                            target_urls.insert(clean_str.clone());
                            if !visited.contains(&clean_str) {
                                to_visit.push(clean_link);
                            }
                        } else {
                            // サブドメイン違い
                            subdomain_urls.insert(clean_str);
                        }
                    }
                }
            }
        }
    }

    Ok((target_urls, subdomain_urls))
}
