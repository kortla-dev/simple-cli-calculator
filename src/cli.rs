use std::env;

pub(crate) fn parse_input(mut args: env::Args) -> String {
    // skip program name
    args.next();

    if args.len() != 1 {
        panic!("Error: Expected 1 argument, but got {}", args.len());
    }

    args.next().unwrap().to_string()
}
