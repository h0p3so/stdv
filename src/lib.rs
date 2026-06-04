use std::collections::HashSet;

#[derive(Debug)]
pub enum Error {
    DupShortname(usize),
    DupLongname(usize),
    AnonymousFlag(usize),
    InvalidShortname(usize),
}

pub struct Flag {
    pub shortname: Option<char>,
    pub longname: Option<&'static str>,
    pub seen: bool,
}

#[derive(Debug)]
pub struct Argrs {
    pub positional: Vec<String>,
}

impl Flag {
    pub fn default () -> Self {
        Self {
            shortname: None,
            longname: None,
            seen: false,
        }
    }
}

/* makes sure all the flags defined make sense as an unit and as group memeber
 * checks:
 * - has either a shortname or longname
 * - there are not flags with the same shortname or longname
 * - shortnames is valid (A-Za-z0-9)
 */
fn check_integrity (flags: &[Flag]) -> Result<(), Error> {
    let mut shortmapper: HashSet<char> = HashSet::new();
    let mut longmapper : HashSet<&str> = HashSet::new();

    for (i, flag) in flags.iter().enumerate() {
        if flag.shortname.is_none() && flag.longname.is_none() {
            return Err(Error::AnonymousFlag(i));
        }

        if let Some(shrtname) = flag.shortname {
            if !shortmapper.insert(shrtname) {
                return Err(Error::DupShortname(i));
            }
            if !shrtname.is_ascii_alphanumeric() {
                return Err(Error::InvalidShortname(i));
            }
        }

        if let Some(longname) = flag.longname.as_deref() {
            if !longmapper.insert(longname) {
                return Err(Error::DupLongname(i));
            }
        }
    }

    Ok(())
}

pub fn argrs (args: Vec<String>, flags: &mut [Flag]) -> Result<Argrs, Error> {
    check_integrity(flags) ?;

    Ok(Argrs { positional: Vec::new() })
}
