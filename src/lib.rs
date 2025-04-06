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
            content: {
                if !sensitivity {
                    self.content.to_lowercase()
                } else {
                    self.content
                }
            },
            case_sens: sensitivity,
            autocorrect: true,
        }
        self
    }

    /// Sets autocorrect to the desired setting
    pub fn autocorrect(self, autocorrect: bool) -> Self {
        Self {
            content: self.content,
            case_sens: self.case_sens,
            autocorrect,
        }
    }

    /// Search through the provided context (of the Data struct)
    /// Currently, doesn't support multi word search (ex: searching for an instance of "two words")
    pub fn search(self, search_target: String, margin: u8) -> Option<(Vec<String>, u32)> {
        let target = search_target.as_str().trim();
        let mut target_counter = 0;
        let mut result = Vec::new();

        if !self.content.contains(target) {
            return None;
        }

        for line in self.content.lines() {
            if line.contains(target) {
                //

                let word_count = line.split_whitespace().count();
                let min_word_count = (2 * margin) + 1;

                //let mut temp_vec = vec![];
                //let mut temp_vec_counter = 0;
                //let mut word_pos: i32;

                for word in line.split_whitespace() {
                    if word.contains(target) {
                        target_counter += 1
                    }
                    result.push(line.to_string());
                    if word_count <= min_word_count as usize {

                        // } else {
                        //     temp_vec.push(word);

                        //     if word.contains(target) {
                        //         word_pos = temp_vec_counter;
                        //     }

                        //     temp_vec_counter += 1;
                    }
                }
            }
        }

        Some((result, target_counter))
    }
}
