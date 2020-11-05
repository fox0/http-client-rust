/// используется только в release-сборке
pub mod panic {
    use std::panic;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::ops::Deref;

    #[allow(dead_code)]
    /// кастомный хук на панику. Пишет в консоль и файл ERRORS.log
    pub fn set_panic_hook() {
        panic::set_hook(Box::new(|panic_info| {
            let (filename, line) =
                panic_info.location().map(|loc| (loc.file(), loc.line()))
                    .unwrap_or(("<unknown>", 0));
            let cause = panic_info.payload().downcast_ref::<String>().map(String::deref)
                .unwrap_or_else(|| panic_info.payload().downcast_ref::<&str>().map(|s| *s)
                    .unwrap_or("<cause unknown>"));

            println!("[ERROR] {}:{}: {}", filename, line, cause);

            // create a new file, or open it if it already exists.
            let mut file = OpenOptions::new().append(true).create(true).
                open("ERRORS.log").unwrap();
            writeln!(file, "{}:{}: {}", filename, line, cause).unwrap();
        }));
    }
}
