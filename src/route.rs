pub struct Route<'a> {
    pub path: &'a str,

    #[cfg(feature = "regex")]
    pub constraints: Vec<(&'a str, &'a str)>,
}

impl<'a> Route<'a> {
    #[must_use]
    pub const fn new(path: &'a str) -> Self {
        Self {
            path,

            #[cfg(feature = "regex")]
            constraints: vec![],
        }
    }
}

impl<'a> From<&'a str> for Route<'a> {
    fn from(path: &'a str) -> Self {
        Self::new(path)
    }
}

impl<'a> From<(&'a str, Vec<(&'a str, &'a str)>)> for Route<'a> {
    fn from((path, constraints): (&'a str, Vec<(&'a str, &'a str)>)) -> Self {
        Self { path, constraints }
    }
}
