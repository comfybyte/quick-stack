use anyhow::{anyhow, Result};
use clap::ArgMatches;

pub fn add(args: &ArgMatches) -> Result<()> {
    let matching = match args.try_get_one::<String>("matching") {
        Ok(Some(matching)) => matching.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };
    let from = match args.try_get_one::<String>("from") {
        Ok(Some(from)) => from.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };
    let to = match args.try_get_one::<String>("to") {
        Ok(Some(to)) => to.clone(),
        Ok(None) => unreachable!(),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };

    super::add(matching, from.into(), to.into())?;
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
