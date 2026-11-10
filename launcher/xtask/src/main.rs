//! Regenerate catalog.json from the public GitHub Releases of this repository.

use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use sha2::Digest;
use smithcc_catalog::{DEFAULT_REPO, ReleaseAsset, ReleaseRecord, app_meta, assemble, releases_from_github_json};

#[derive(Parser)]
#[command(name = "xtask", about = "Smith Creative Cloud release helpers")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand)]
enum Cmd {
    /// Write catalog.json from GitHub Releases.
    Catalog {
        /// owner/name. Defaults to mahmudurrahman420/smith-suite.
        #[arg(long, default_value = DEFAULT_REPO)]
        repo: String,
        #[arg(long, default_value = "catalog.json")]
        out: PathBuf,
        /// Read a GitHub releases JSON array instead of calling the API.
        #[arg(long)]
        from_file: Option<PathBuf>,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("xtask: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    match Cli::parse().cmd {
        Cmd::Catalog { repo, out, from_file } => catalog(&repo, &out, from_file.as_deref()),
    }
}

fn catalog(repo: &str, out: &std::path::Path, from_file: Option<&std::path::Path>) -> Result<(), String> {
    let records = if let Some(path) = from_file {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        releases_from_github_json(&value).map_err(|e| e.to_string())?
    } else {
        let raw = fetch_releases(repo)?;
        let mut records = releases_from_github_json(&serde_json::Value::Array(raw)).map_err(|e| e.to_string())?;
        fill_missing_digests(&mut records)?;
        records
    };
    let apps = app_meta()?;
    let generated_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let catalog = assemble(&records, &apps, &generated_at, repo);
    let body = serde_json::to_vec_pretty(&catalog).map_err(|e| e.to_string())?;
    if let Some(parent) = out.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(out, body).map_err(|e| e.to_string())?;
    eprintln!("wrote {} ({} apps)", out.display(), catalog.apps.len());
    Ok(())
}

fn fetch_releases(repo: &str) -> Result<Vec<serde_json::Value>, String> {
    let client = reqwest::blocking::Client::builder().user_agent("SmithCreativeCloud-catalog").build().map_err(|e| e.to_string())?;
    let mut url = format!("https://api.github.com/repos/{repo}/releases?per_page=100");
    let mut all = Vec::new();
    for _ in 0..20 {
        let mut request = client.get(&url).header("Accept", "application/vnd.github+json").header("X-GitHub-Api-Version", "2022-11-28");
        if let Ok(token) = std::env::var("GITHUB_TOKEN").or_else(|_| std::env::var("GH_TOKEN"))
            && !token.is_empty()
        {
            request = request.bearer_auth(token);
        }
        let response = request.send().map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            return Err(format!("GitHub releases returned {}", response.status()));
        }
        let next = next_link(response.headers().get("link").and_then(|v| v.to_str().ok()));
        let page: Vec<serde_json::Value> = response.json().map_err(|e| e.to_string())?;
        if page.is_empty() {
            break;
        }
        all.extend(page);
        match next {
            Some(link) => url = link,
            None => break,
        }
    }
    Ok(all)
}

fn next_link(header: Option<&str>) -> Option<String> {
    let header = header?;
    for part in header.split(',') {
        let mut pieces = part.split(';');
        let url = pieces.next()?.trim().trim_start_matches('<').trim_end_matches('>').to_string();
        let rel = pieces.next().unwrap_or("").trim();
        if rel.contains("rel=\"next\"") {
            return Some(url);
        }
    }
    None
}

fn fill_missing_digests(records: &mut [ReleaseRecord]) -> Result<(), String> {
    let client = reqwest::blocking::Client::builder().user_agent("SmithCreativeCloud-catalog").build().map_err(|e| e.to_string())?;
    for record in records {
        for asset in &mut record.assets {
            if asset.sha256.len() == 64 {
                continue;
            }
            asset.sha256 = hash_url(&client, asset)?;
        }
    }
    Ok(())
}

fn hash_url(client: &reqwest::blocking::Client, asset: &ReleaseAsset) -> Result<String, String> {
    let mut response = client.get(&asset.url).send().map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("download {} returned {}", asset.name, response.status()));
    }
    let mut hasher = sha2::Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    let mut total = 0u64;
    loop {
        let n = response.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        total = total.saturating_add(n as u64);
        if total > 2_200_000_000 {
            return Err(format!("{} is larger than 2 GB", asset.name));
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex_encode(hasher.finalize().as_slice()))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}
