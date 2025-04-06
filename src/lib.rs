#[derive(Clone)]
#[allow(unused)]
pub struct Data {
    content: String,
    case_sens: bool,
    autocorrect: bool,
}

impl Data {
    /// Creates the default implementation of the Data impl with autocorrect and case_sensitivity being on
    pub fn new(content: String) -> Self {
        Self {
            content,
            case_sens: true,
            autocorrect: true,
        }
    }

    /// Sets the case sensitivity to the desired setting
    pub fn case_sens(self, sensitivity: bool) -> Self {
        Self {
            content: self.content,
            case_sens: sensitivity,
            autocorrect: true,
        }
    }

    /// Sets autocorrect to the desired setting
    pub fn autocorrect(self, autocorrect: bool) -> Self {
        Self {
            content: self.content,
            case_sens: self.case_sens,
            autocorrect,
        }
    }

    /// Search through the provided content (of the Data struct)
    pub fn search(self, search_target: String) -> Option<(Vec<String>, u32)> {
        let target = search_target.as_str().trim();
        let mut target_counter = 0;
        let mut result = Vec::new();

        if !self.content.contains(target) {
            return None;
        }

        for line in self.content.lines() {
            let line_contains_target = if self.case_sens {
                line.contains(target)
            } else {
                line.to_lowercase().contains(target.to_lowercase().as_str())
            };

            if line_contains_target {
                // Add the line only once
                result.push(line.to_string());

                // Count occurrences in this line
                for word in line.split_whitespace() {
                    if word.contains(target) {
                        target_counter += 1;
                    }
                }
            }
        }

        Some((result, target_counter))
    }
}
