use crate::{errors::QSError, rules::Rulefile};

pub fn clear() -> Result<(), QSError> {
    let mut rulefile = Rulefile::load()?;
    rulefile.rules.clear();
    rulefile.commit()?;

    println!("cleared all rules.");
    Ok(())
}
