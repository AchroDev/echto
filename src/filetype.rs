pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default, Clone, Copy)]
pub struct HighlightingOptions {
    numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.hl_opts
    }

    #[must_use]
    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions { numbers: true },
            };
        }
        if file_name.ends_with(".kt") {
            return Self {
                name: String::from("Kotlin"),
                hl_opts: HighlightingOptions { numbers: true },
            };
        } else if file_name.ends_with(".kts") {
            return Self {
                name: String::from("Kotlin"),
                hl_opts: HighlightingOptions { numbers: true },
            };
        }
        if file_name.ends_with(".go") {
            return Self {
                name: String::from("Go"),
                hl_opts: HighlightingOptions { numbers: true },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }
}
