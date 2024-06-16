use clap::ArgMatches;

use crate::errors::QSError;

pub fn add(args: &ArgMatches) -> Result<(), QSError> {
    let matching = match args.try_get_one::<String>("matching") {
        Ok(Some(matching)) => matching.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(QSError::ParseFailed("matching".to_string(), err)),
    };
    let input = match args.try_get_one::<String>("input") {
        Ok(Some(from)) => from.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(QSError::ParseFailed("input".to_string(), err)),
    };
    let output = match args.try_get_one::<String>("output") {
        Ok(Some(to)) => to.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(QSError::ParseFailed("output".to_string(), err)),
    };

    super::add(matching, input.into(), output.into())
}

pub fn rm(args: &ArgMatches) -> Result<(), QSError> {
    let numbers: Vec<usize> = match args.try_get_many::<String>("numbers") {
        Ok(Some(numbers)) => numbers.flat_map(|n| n.parse::<usize>()).collect(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(QSError::ParseFailed("numbers".to_string(), err)),
    };

    if numbers.is_empty() {
        println!("nothing to do.");
    } else {
        super::rm(&numbers)?;
    }

    Ok(())
}
