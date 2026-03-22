use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "gos-verify", version, about = "Verify GrapheneOS releases")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Verify a downloaded factory image or OTA file
    Verify {
        /// Path to the release file (factory.zip or ota_update.zip)
        file: PathBuf,

        /// Device codename (e.g., husky, tokay, caiman)
        #[arg(short, long)]
        device: String,
    },

    /// Check latest release for a device
    Latest {
        /// Device codename
        device: String,

        /// Release channel (stable, beta, testing)
        #[arg(short, long, default_value = "stable")]
        channel: String,
    },

    /// List supported devices
    Devices,

    /// Hash a file with BLAKE3
    Hash {
        /// File to hash
        file: PathBuf,
    },
}

/// Supported GrapheneOS devices.
const DEVICES: &[(&str, &str)] = &[
    ("oriole", "Pixel 6"),
    ("raven", "Pixel 6 Pro"),
    ("bluejay", "Pixel 6a"),
    ("panther", "Pixel 7"),
    ("cheetah", "Pixel 7 Pro"),
    ("lynx", "Pixel 7a"),
    ("felix", "Pixel Fold"),
    ("tangorpro", "Pixel Tablet"),
    ("shiba", "Pixel 8"),
    ("husky", "Pixel 8 Pro"),
    ("akita", "Pixel 8a"),
    ("tokay", "Pixel 9"),
    ("caiman", "Pixel 9 Pro"),
    ("komodo", "Pixel 9 Pro XL"),
    ("comet", "Pixel 9 Pro Fold"),
    ("tegu", "Pixel 9a"),
    ("stallion", "Pixel 10"),
    ("rango", "Pixel 10 Pro"),
    ("mustang", "Pixel 10 Pro XL"),
    ("blazer", "Pixel 10 Pro Fold"),
];

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Verify { file, device } => {
            if !file.exists() {
                eprintln!("error: file not found: {}", file.display());
                return ExitCode::FAILURE;
            }

            if !DEVICES.iter().any(|(d, _)| *d == device) {
                eprintln!("error: unknown device '{}'. Use 'gos-verify devices' to list.", device);
                return ExitCode::FAILURE;
            }

            println!("Verifying {} for {}...", file.display(), device);

            // Hash the file
            let data = match std::fs::read(&file) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::FAILURE;
                }
            };

            let hash = blake3::hash(&data);
            println!("BLAKE3: {}", hash.to_hex());
            println!("Size: {} bytes ({:.1} MB)", data.len(), data.len() as f64 / 1_048_576.0);

            // Check ZIP structure
            if data.len() >= 4 && &data[..4] == b"PK\x03\x04" {
                println!("Format: Valid ZIP archive");
            } else {
                eprintln!("warning: file does not appear to be a ZIP archive");
            }

            // Check for GrapheneOS markers in filename
            let filename = file.file_name().unwrap_or_default().to_string_lossy();
            if filename.contains(&device) {
                println!("Device match: filename contains '{device}'");
            } else {
                eprintln!("warning: filename does not contain device codename '{device}'");
            }

            if filename.contains("factory") {
                println!("Type: Factory image");
            } else if filename.contains("ota_update") {
                println!("Type: Full OTA update");
            } else if filename.contains("incremental") {
                println!("Type: Incremental OTA update");
            } else {
                println!("Type: Unknown");
            }

            println!("\nVerification: BLAKE3 hash computed successfully.");
            println!("Compare against: https://releases.grapheneos.org/{device}-stable");

            ExitCode::SUCCESS
        }

        Command::Latest { device, channel } => {
            if !DEVICES.iter().any(|(d, _)| *d == device) {
                eprintln!("error: unknown device '{}'. Use 'gos-verify devices' to list.", device);
                return ExitCode::FAILURE;
            }

            let url = format!("https://releases.grapheneos.org/{device}-{channel}");
            println!("Checking: {url}");

            match reqwest::blocking::get(&url) {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.text() {
                            Ok(body) => {
                                let build = body.trim();
                                println!("Device: {device}");
                                println!("Channel: {channel}");
                                println!("Latest build: {build}");
                                println!("\nFactory: https://releases.grapheneos.org/{device}-factory-{build}.zip");
                                println!("OTA: https://releases.grapheneos.org/{device}-ota_update-{build}.zip");
                            }
                            Err(e) => {
                                eprintln!("error reading response: {e}");
                                return ExitCode::FAILURE;
                            }
                        }
                    } else {
                        eprintln!("error: HTTP {} — no {channel} release for {device}", resp.status());
                        return ExitCode::FAILURE;
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::FAILURE;
                }
            }

            ExitCode::SUCCESS
        }

        Command::Devices => {
            println!("Supported GrapheneOS devices:\n");
            for (codename, name) in DEVICES {
                println!("  {codename:12} {name}");
            }
            ExitCode::SUCCESS
        }

        Command::Hash { file } => {
            match std::fs::read(&file) {
                Ok(data) => {
                    let hash = blake3::hash(&data);
                    println!("{}", hash.to_hex());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
