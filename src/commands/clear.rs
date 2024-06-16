use crate::{errors::QSError, Rulefile};

pub fn clear() -> Result<(), QSError> {
    let mut rulefile = Rulefile::load()?;
    rulefile.rules.clear();
    rulefile.save()?;

    println!("cleared all rules.");
    Ok(())
}
