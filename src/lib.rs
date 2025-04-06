pub struct Data {
    content: String,
    case_sens: bool,
    autocorrect: bool,
}

impl Data {
    /// Creates the default implementation of the Data impl with autocorrect and case_sensitivity being on
    fn new(content: String) -> Self {
        Self {
            content,
            case_sens: true,
            autocorrect: true,
        }
    }

    /// Sets the case sensitivity to the desired setting
    fn case_sens(&mut self, sensitivity: bool) -> &mut Self {
        self.case_sens = sensitivity;
        if !sensitivity {
            self.content = self.content.to_lowercase();
        }
        self
    }

    /// Sets autocorrect to the desired setting
    fn autocorrect(&mut self, autocorrect: bool) -> &mut Self {
        self.autocorrect = autocorrect;
        self
    }
}
