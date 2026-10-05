use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};

pub fn save_list(filename: &str, urls: &HashSet<String>) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let mut sorted_urls: Vec<_> = urls.iter().collect();
    sorted_urls.sort();

    for url in sorted_urls {
        writeln!(file, "{}", url)?;
    }
    println!("[+] Saved {} URLs to {}", urls.len(), filename);
    Ok(())
}
