use anyhow::{anyhow, Result};
use clap::ArgMatches;

pub fn add(args: &ArgMatches) -> Result<()> {
    let matching = match args.try_get_one::<String>("matching") {
        Ok(Some(matching)) => matching.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };
    let input = match args.try_get_one::<String>("input") {
        Ok(Some(from)) => from.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };
    let output = match args.try_get_one::<String>("output") {
        Ok(Some(to)) => to.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };

    super::add(matching, input.into(), output.into())?;
    Ok(())
}

pub fn rm(args: &ArgMatches) -> Result<()> {
    let numbers = match args.try_get_many::<String>("numbers") {
        Ok(Some(numbers)) => numbers.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };
    let numbers: Vec<usize> = numbers.flat_map(|n| n.parse::<usize>()).collect();

    super::rm(&numbers)?;
    Ok(())
}
