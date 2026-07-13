// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let msg = format!(
            "Panic: {}\nLocation: {:?}\nBacktrace:\n{:?}",
            info.to_string(),
            info.location(),
            std::backtrace::Backtrace::capture(),
        );
        let path = std::env::temp_dir().join("lucid-spell-panic.log");
        let _ = std::fs::write(&path, &msg);
        // Also print to stderr — visible in terminal or debug builds
        eprintln!("{}", msg);
    }));

    lucid_spell_lib::run()
}
