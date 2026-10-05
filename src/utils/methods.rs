use std::fs;
use std::io;
use std::process::{Command, Stdio};

pub fn run_nuclei(target_file: &str) -> io::Result<()> {
    // skip this if file is not here or empty
    if let Ok(metadata) = fs::metadata(target_file) {
        if metadata.len() == 0 {
            println!("[-] {} is empty. Skipping Nuclei scan.", target_file);
            return Ok(());
        }
    } else {
        println!("[-] {} does not exist. Skipping Nuclei scan.", target_file);
        return Ok(());
    }

    println!(
        "[*] Executing Nuclei scan using targets in {}...\n",
        target_file
    );

    // nuclei -list targetlist.txt -silent -severity info,low,medium,high,critical -tags cve
    let mut child = Command::new("nuclei")
        .arg("-list")
        .arg(target_file)
        .arg("-silent")
        .arg("-severity")
        .arg("info,low,medium,high,critical")
        .arg("-tags")
        .arg("cve")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    child.wait()?;
    Ok(())
}
