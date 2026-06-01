pub enum GRSError {
    InvalidShortname,
    DuplicatedShortname,
    DuplicatedLongname,
    MalformedOpts,
    
    UnknownShortname,
    MissingArgument,
    NonsenseArgument,
    UnknownLongname,
    TooManyPosArgs
}

impl GRSError {

}

pub fn grs_init () {
    println!("Hello, world!");
}
