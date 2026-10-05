mod utils;

use clap::Parser;
use url::Url;
use utils::domain_collect::crawl;
use utils::filemake::save_list;
use utils::methods::run_nuclei;

/// PeaceMaker - Web Vulnerability Scanner Automation Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target URL (e.g. https://example.com)
    target_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. clap による引数解析
    let args = Args::parse();
    let target_url = Url::parse(&args.target_url)?;

    let target_host = match target_url.host_str() {
        Some(h) => h.to_string(),
        None => {
            eprintln!("[!] Invalid host in provided URL.");
            std::process::exit(1);
        }
    };

    // ルートドメインの簡易抽出 (例: sub.example.com -> example.com)
    let host_parts: Vec<&str> = target_host.split('.').collect();
    let root_domain = if host_parts.len() >= 2 {
        format!(
            "{}.{}",
            host_parts[host_parts.len() - 2],
            host_parts[host_parts.len() - 1]
        )
    } else {
        target_host.clone()
    };

    println!("[*] Target URL: {}", target_url);

    // 2. クローリング実行 (domain_collect)
    let (target_urls, subdomain_urls) = crawl(&target_url, &target_host, &root_domain).await?;

    // 3. ファイル作成 (filemake)
    save_list("targetlist.txt", &target_urls)?;
    save_list("subdomain_list.txt", &subdomain_urls)?;

    // 4. nucleiスキャン実行 (methods)
    run_nuclei("targetlist.txt")?;

    Ok(())
}
