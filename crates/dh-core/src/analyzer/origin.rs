//! Where a process comes from, read from the path of its executable.
//!
//! The name catalogue knows about a hundred processes; a Mac runs several
//! hundred, so nearly everything was "Unknown". The path answers the question
//! the README asks, "can I get rid of this?", for all of them: a file under
//! /System belongs to the operating system, one inside Foo.app belongs to Foo.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    /// Part of the operating system; not something to remove.
    OperatingSystem,
    /// Belongs to an installed application, named here.
    App(String),
    /// Installed outside the system and outside any app bundle, e.g. a helper
    /// tool or a script in a home directory.
    ThirdParty,
}

pub fn origin_of(exe: &str) -> Origin {
    if let Some(app) = app_bundle_name(exe) {
        // Apple's own apps (Mail, Notes) live under /System/Applications and can be
        // quit like any app; bundles elsewhere under /System (Finder, Dock) cannot.
        let system_internal = exe.starts_with("/System/") && !exe.starts_with("/System/Applications/");
        return if system_internal { Origin::OperatingSystem } else { Origin::App(app) };
    }
    if is_system_path(exe) {
        return Origin::OperatingSystem;
    }
    if let Some(vendor) = program_files_vendor(exe).or_else(|| opt_package(exe)) {
        return Origin::App(vendor);
    }
    Origin::ThirdParty
}

/// The outermost `.app` bundle, so helpers of an app report the app itself:
/// ".../Google Chrome.app/.../Google Chrome Helper.app/..." → "Google Chrome".
fn app_bundle_name(exe: &str) -> Option<String> {
    exe.split('/')
        .find(|part| part.ends_with(".app") && part.len() > 4)
        .map(|part| part.trim_end_matches(".app").to_string())
}

fn is_system_path(exe: &str) -> bool {
    const UNIX: &[&str] = &[
        "/System/", "/usr/libexec/", "/usr/sbin/", "/usr/bin/", "/sbin/", "/bin/",
        "/usr/lib/", "/lib/", "/Library/Apple/",
    ];
    let lower = exe.to_lowercase();
    UNIX.iter().any(|prefix| exe.starts_with(prefix)) || lower.starts_with("c:\\windows\\")
}

/// `C:\Program Files\Vendor\...` → "Vendor".
fn program_files_vendor(exe: &str) -> Option<String> {
    let lower = exe.to_lowercase();
    let rest = ["c:\\program files (x86)\\", "c:\\program files\\"]
        .iter()
        .find(|prefix| lower.starts_with(*prefix))
        .map(|prefix| &exe[prefix.len()..])?;
    rest.split('\\').next().filter(|v| !v.is_empty()).map(str::to_string)
}

/// `/opt/<name>/...` and `/snap/<name>/...` on Linux.
fn opt_package(exe: &str) -> Option<String> {
    ["/opt/", "/snap/"]
        .iter()
        .find(|prefix| exe.starts_with(*prefix))
        .and_then(|prefix| exe[prefix.len()..].split('/').next())
        // Homebrew is a package manager, not the app the process belongs to.
        .filter(|name| !name.is_empty() && *name != "homebrew")
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_helpers_belong_to_their_app() {
        let exe = "/Applications/Google Chrome.app/Contents/Frameworks/Google Chrome Framework.framework/Helpers/Google Chrome Helper.app/Contents/MacOS/Google Chrome Helper";
        assert_eq!(origin_of(exe), Origin::App("Google Chrome".into()));
    }

    #[test]
    fn system_files_and_system_apps_are_the_operating_system() {
        assert_eq!(origin_of("/usr/libexec/trustd"), Origin::OperatingSystem);
        assert_eq!(origin_of("/System/Library/CoreServices/Finder.app/Contents/MacOS/Finder"), Origin::OperatingSystem);
        assert_eq!(origin_of("C:\\Windows\\System32\\svchost.exe"), Origin::OperatingSystem);
        assert_eq!(origin_of("/usr/lib/systemd/systemd-journald"), Origin::OperatingSystem);
        assert_eq!(origin_of("/System/Applications/Mail.app/Contents/MacOS/Mail"), Origin::App("Mail".into()));
    }

    #[test]
    fn installed_programs_name_their_vendor_or_package() {
        assert_eq!(origin_of("C:\\Program Files\\Mozilla Firefox\\firefox.exe"), Origin::App("Mozilla Firefox".into()));
        assert_eq!(origin_of("/opt/zoom/zoom"), Origin::App("zoom".into()));
        assert_eq!(origin_of("/snap/firefox/123/usr/lib/firefox/firefox"), Origin::App("firefox".into()));
    }

    #[test]
    fn everything_else_is_third_party() {
        assert_eq!(origin_of("/Library/PrivilegedHelperTools/com.example.helper"), Origin::ThirdParty);
        assert_eq!(origin_of("/opt/homebrew/bin/ollama"), Origin::ThirdParty);
        assert_eq!(origin_of("/Users/me/bin/tool"), Origin::ThirdParty);
    }
}
