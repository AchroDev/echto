pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default, Clone, Copy)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
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
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                },
            };
        }
        if file_name.ends_with(".kt") {
            return Self {
                name: String::from("Kotlin"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                },
            };
        } else if file_name.ends_with(".kts") {
            return Self {
                name: String::from("Kotlin"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                },
            };
        }
        if file_name.ends_with(".js") {
            return Self {
                name: String::from("JavaScript"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                },
            };
        } else if file_name.ends_with(".jsx") {
            return Self {
                name: String::from("JavaScript XML (React)"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                },
            };
        }
        if file_name.ends_with(".go") {
            return Self {
                name: String::from("Go"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    #[must_use]
    pub fn numbers(self) -> bool {
        self.numbers
    }

    #[must_use]
    pub fn strings(self) -> bool {
        self.strings
    }

    #[must_use]
    pub fn characters(self) -> bool {
        self.characters
    }

    #[must_use]
    pub fn comments(self) -> bool {
        self.comments
    }
}
