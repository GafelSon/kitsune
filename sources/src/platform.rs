pub fn enable_utf8_terminal() {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "chcp 65001"])
            .output();
    }
}
