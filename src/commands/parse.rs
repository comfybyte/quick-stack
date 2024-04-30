use anyhow::{anyhow, Result};
use clap::ArgMatches;

pub fn add(args: &ArgMatches) -> Result<()> {
    let matching = match args.try_get_one::<String>("matching") {
        Ok(Some(matching)) => matching.clone(),
        Ok(None) => return Err(anyhow!("missing matching.")),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };
    let from = match args.try_get_one::<String>("from") {
        Ok(Some(from)) => from.clone(),
        Ok(None) => return Err(anyhow!("missing from.")),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };
    let to = match args.try_get_one::<String>("to") {
        Ok(Some(to)) => to.clone(),
        Ok(None) => return Err(anyhow!("missing to.")),
        Err(err) => return Err(anyhow!("parsing error: {err:?}")),
    };

    super::add(matching, from.into(), to.into())?;
    Ok(())
}
