use anyhow::Result;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

pub mod args;
pub mod commands;

#[derive(Debug)]
pub struct Rule {
    matching: String,
    from: PathBuf,
    to: PathBuf,
}
impl From<&Rule> for String {
    fn from(value: &Rule) -> Self {
        format!(
            "{}\n{}\n{}",
            value.matching,
            value.from.display(),
            value.to.display()
        )
    }
}

#[derive(Debug, Default)]
pub struct Rulefile {
    rules: Vec<Rule>,
}
impl Rulefile {
    /// Gets the path of the `rulefile`.
    fn default_path() -> Result<PathBuf> {
        Ok(xdg_dirs()?.place_data_file("rulefile")?)
    }
    /// Reads the `rulefile` and parses it into a manipulatable instance.
    /// Call `Self.save` to write any changes to disk.
    ///
    /// # Errors
    /// If the `rulefile` can't be accessed, created or if it can't be parsed (from malformatting).
    pub fn new() -> Result<Self> {
        Self::read_as_string()?.try_into()
    }
    /// Reads the `rulefile` into a string.
    pub fn read_as_string() -> Result<String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(Self::default_path()?)?
            .bytes()
            .flatten()
            .collect::<Vec<u8>>();
        let content = String::from_utf8(file)?;

        Ok(content)
    }
    /// Overwrites the `rulefile` with this instance's.
    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Self::default_path()?)?;
        file.write_all(String::from(self).as_bytes())?;

        Ok(())
    }
    /// Sugar over pushing to `self.rules` to allow chaining.
    #[must_use]
    pub fn push(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }
}

const BROKEN_RULE: &str =
    "malformatted rule found. use `edit` to manually fix it or `clear` to start over.";

impl TryFrom<String> for Rulefile {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::prelude::v1::Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Self::default())
        } else {
            let rules: Vec<Rule> = value
                .split("\n\n")
                .map(|chunk| {
                    let mut lines = chunk.lines();

                    Rule {
                        // TODO: Use `return`s instead of panicking like an idiot.
                        matching: lines.next().expect(BROKEN_RULE).into(),
                        from: lines.next().expect(BROKEN_RULE).into(),
                        to: lines.next().expect(BROKEN_RULE).into(),
                    }
                })
                .collect();

            Ok(Self { rules })
        }
    }
}

impl From<&Rulefile> for String {
    fn from(value: &Rulefile) -> Self {
        value
            .rules
            .iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

fn xdg_dirs() -> Result<xdg::BaseDirectories> {
    Ok(xdg::BaseDirectories::with_prefix("quick-stack")?)
}
