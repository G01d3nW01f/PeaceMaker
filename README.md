# PeaceMaker

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)

**PeaceMaker** is a CLI tool designed to streamline and automate web vulnerability assessments for internal systems and group company assets.

It automatically crawls internal links starting from a target URL, constructs an in-scope target list, and runs automated security scans using [Nuclei](https://github.com/projectdiscovery/nuclei).

---

## Features

- 🔍 **Automated Crawling**: Discovers pages within the same host and exports them to `targetlist.txt`.
- 🌐 **Subdomain Separation**: Detects URLs belonging to different subdomains under the same root domain and isolates them into `subdomain_list.txt`.
- 🚫 **External Link Filtering**: Automatically filters out third-party external domains.
- ⚡ **Nuclei Integration**: Executes targeted security scans filtered for CVEs and high-severity vulnerabilities.
- 🚀 **Built with Rust**: Fast execution, safe concurrency, and single-binary portability.

---

## Prerequisites

Before running PeaceMaker, ensure the following tools are installed on your system:

1. **Rust / Cargo** (for building from source)
   - [Official Installation Guide](https://www.rust-lang.org/tools/install)
2. **Nuclei**
   - [ProjectDiscovery Nuclei Installation Guide](https://github.com/projectdiscovery/nuclei#installation)
   - Verify that it is available in your PATH (`nuclei -version`)

---

## Installation

Build the binary from the source code:

```bash
# Clone the repository
git clone [https://github.com/](https://github.com/)<your-username>/peacemaker.git
cd peacemaker

# Build for release
cargo build --release

# Copy the binary to a PATH directory (optional)
sudo cp target/release/peacemaker /usr/local/bin/
```

---

## Usage

Basic command structure:

```text
peacemaker <TARGET_URL>
```

### Example Command

```text
peacemaker [http://10.48.145.130](http://10.48.145.130)
```

### Output Example

```text
[*] Target URL: [http://10.48.145.130/](http://10.48.145.130/)
[+] Saved 2 URLs to targetlist.txt
[+] Saved 0 URLs to subdomain_list.txt
[*] Executing Nuclei scan using targets in targetlist.txt...

[CVE-2018-16763] [http] [critical] [http://10.48.145.130/fuel/pages/select/?filter=%27%2bpi(print(%24a%3d%27system%27))%2b%24a(%27cat%20/etc/passwd%27)%2b%27](http://10.48.145.130/fuel/pages/select/?filter=%27%2bpi(print(%24a%3d%27system%27))%2b%24a(%27cat%20/etc/passwd%27)%2b%27)
```

---

## Output Files

After execution, the following files will be created in your current working directory:

| Filename | Description |
| :--- | :--- |
| `targetlist.txt` | Collected URLs matching the target host (scanned by Nuclei) |
| `subdomain_list.txt` | Discovered URLs belonging to different subdomains under the same root domain |

---

## Project Structure

```text
peacemaker/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── utils/
        ├── client.rs          # HTTP client setup
        ├── domain_collect.rs  # Crawling & domain analysis logic
        ├── filemake.rs        # File I/O operations for target lists
        ├── methods.rs         # Nuclei subprocess execution
        └── mod.rs
```

---

## Disclaimer

This tool is created solely for **authorized security testing and educational assessment of systems owned by or explicitly permitted by the target system owner**.
Executing this tool against third-party systems without prior authorization may violate applicable local and international laws. The author assumes no liability for any misuse or damages caused by this tool.

---

## License

[MIT License](LICENSE)
