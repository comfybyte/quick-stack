use crate::{
    errors::QSError,
    rules::{OutOfBoundsError, Rulefile},
};

pub fn rm(numbers: &[usize]) -> Result<(), QSError> {
    let mut rulefile = Rulefile::load()?;
    let mut should_commit = false;

    for n in numbers {
        match rulefile.remove_rule(*n) {
            Ok(()) => {
                println!("removed rule #{}.", *n);
                should_commit = true;
            }
            Err(OutOfBoundsError::Zero) => eprintln!("can't remove rule #0, as rules start at #1."),
            Err(OutOfBoundsError::Overflow) => {
                eprintln!("can't remove rule #{}, as it doesn't exist.", *n);
            }
        };
    }

    if should_commit {
        rulefile.commit()?;
        println!("done.");
    } else {
        println!("nothing to do.");
    }
    Ok(())
}
