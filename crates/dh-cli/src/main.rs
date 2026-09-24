use anyhow::Result;
use dh_core::analyzer::{run_full_analysis, AnalysisResult};
use dh_core::models::{process::ProcessCategory, settings::AppSettings, Lang};

/// The CLI speaks the language of the shell: LANG=de_CH.UTF-8 gives German texts.
fn lang() -> Lang {
    std::env::var("LANG").map(|l| Lang::from_code(&l)).unwrap_or_default()
}

async fn analyze() -> Result<AnalysisResult> {
    let settings = AppSettings::default();
    let lang = lang();
    Ok(tokio::task::spawn_blocking(move || run_full_analysis(&settings, lang)).await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "help".into());
    match cmd.as_str() {
        "scan" => {
            let result = analyze().await?;
            println!("Health score: {}/100 ({:?})", result.snapshot.score, result.snapshot.grade);
            println!("Findings: {}", result.findings.len());
            for f in &result.findings {
                println!("  [{:?}] {}: {}", f.severity, f.title, f.affected_item);
            }
            for r in &result.recommendations {
                println!("  -> {}", r.title);
            }
        }
        "processes" => {
            let result = analyze().await?;
            for p in result.processes.iter().filter(|p| !matches!(p.category, ProcessCategory::System)) {
                println!("{:<40} CPU: {:5.1}%  RAM: {:6.0} MB  {:?}  {}",
                    p.name, p.cpu_usage, p.memory_mb(), p.category, p.vendor.as_deref().unwrap_or(""));
            }
        }
        "autostart" => {
            let result = analyze().await?;
            for e in &result.autostart {
                println!("{:<50} {}", e.name, e.location);
            }
        }
        _ => {
            println!("DeviceHealth CLI\n");
            println!("Usage: devicehealth <command>\n");
            println!("Commands:");
            println!("  scan        Full analysis: score, findings, recommendations");
            println!("  processes   Processes that are not part of the system");
            println!("  autostart   Autostart entries");
        }
    }
    Ok(())
}
