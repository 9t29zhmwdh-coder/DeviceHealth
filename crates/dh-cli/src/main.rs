use anyhow::Result;
use dh_core::analyzer::run_full_analysis;
use dh_core::models::settings::AppSettings;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match cmd {
        "scan" => {
            println!("DeviceHealth — Systemanalyse läuft…");
            let settings = AppSettings::default();
            let result = tokio::task::spawn_blocking(move || run_full_analysis(&settings)).await?;
            println!("Gesundheitsscore: {}/100 ({})", result.snapshot.score, result.snapshot.grade.label());
            println!("Gefundene Probleme: {}", result.findings.len());
            for f in &result.findings {
                println!("  [{}] {} — {}", f.severity.label(), f.title, f.affected_item);
            }
        }
        "processes" => {
            let settings = AppSettings::default();
            let result = tokio::task::spawn_blocking(move || run_full_analysis(&settings)).await?;
            for p in result.processes.iter().filter(|p| !matches!(p.category, dh_core::models::process::ProcessCategory::System)) {
                println!("{:<40} CPU: {:5.1}%  RAM: {:6.0} MB  Risk: {:?}",
                    p.name, p.cpu_usage, p.memory_mb(), p.risk);
            }
        }
        _ => {
            println!("DeviceHealth CLI");
            println!();
            println!("Usage: devicehealth <command>");
            println!();
            println!("Commands:");
            println!("  scan        Vollständige Systemanalyse");
            println!("  processes   Alle Prozesse auflisten");
        }
    }
    Ok(())
}
