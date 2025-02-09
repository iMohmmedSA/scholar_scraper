use tracing::warn;
use crate::prelude::*;



pub fn retrieve_list_from_input() -> Result<String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let first_arg = args[1].clone();
        return Ok(first_arg);
    }

    warn!("Failed to read arguments");
    Err(Error::ReadArgs)
}