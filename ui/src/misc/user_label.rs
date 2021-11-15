use ::std::{fmt::Display, ops::Deref};

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
pub struct UserLabel(pub String);

impl Deref for UserLabel {
    type Target = String;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl Display for UserLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
