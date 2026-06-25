struct ParsedHeader<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> ParsedHeader<'a> {
    fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }

    fn is_auth(&self) -> bool {
        self.name == "Authorization"
    }

    fn value_bytes(&self) -> &[u8] {
        // Rule 3: output has 'self lifetime
        self.value.as_bytes()
    }
}

