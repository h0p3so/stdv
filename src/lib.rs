use std::collections::HashSet;

#[derive(Debug)]
pub enum Error {
    DupShortname(usize),
    DupLongname(usize),
    AnonymousFlag(usize),
    InvalidShortname(usize),
    ShouldntExpectedAValue(usize),
    ShouldExpectedAValue(usize),

    UnknownShortname(String, usize),
    UnknownLongname(String, usize, usize),
    BadGrouping(String, usize)
}

#[derive(Copy, Clone, PartialEq)]
pub enum ArgMode {
    Required,
    Optional,
    NoArgument
}

pub enum ArgValue {
    Txt(String),
    F64(   f64),
    I32(   i32),
    U32(   u32),
    I64(   i64),
    U64(   u64)
}

#[derive(Copy, Clone)]
pub enum ArgExpectedType {
    Txt,
    F64,
    I32,
    U32,
    I64,
    U64
}

pub struct Flag {
    pub shortname: Option<char>,
    pub longname:  Option<&'static str>,
    pub value:     Option<ArgValue>,
    pub expected:  Option<ArgExpectedType>,

    pub seen: bool,
    pub mode: ArgMode,
}

#[derive(Debug)]
pub struct Argrs {
    pub flastindex: Option<usize>,
}

impl Flag {
    pub fn default () -> Self {
        Self {
            shortname: None,
            longname : None,
            value    : None,
            expected : None,
            seen     : false,
            mode     : ArgMode::NoArgument
        }
    }
}

/* makes sure all the flags defined make sense as an unit and as group memeber
 * checks:
 * - has either a shortname or longname
 * - there are not flags with the same shortname or longname
 * - shortnames is valid (A-Za-z0-9)
 * - all flags make sense in terms of argument taking
 */
fn check_integrity (flags: &[Flag]) -> Result<(), Error> {
    let mut shortmapper: HashSet<char> = HashSet::new();
    let mut longmapper : HashSet<&str> = HashSet::new();

    for (i, flag) in flags.iter().enumerate() {
        if flag.shortname.is_none() && flag.longname.is_none() {
            return Err(Error::AnonymousFlag(i));
        }

        match (flag.mode, flag.expected) {
            (ArgMode::NoArgument, Some(_expected)) => {
                return Err(Error::ShouldntExpectedAValue(i));
            }
            (ArgMode::Required | ArgMode::Optional, None) => {
                return Err(Error::ShouldExpectedAValue(i));
            }
            _ => {}
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

fn parse_shortopt (source: &String, flags: &mut [Flag]) -> Result<Option<usize>, Error> {
    let mut lastseen: Option<usize> = None;
    let srclen: usize = source.len();

    let srcoffset: usize = 1;
    for (i, shortname) in source.chars().skip(1).enumerate() {
        match flags.iter().position(|f| f.shortname == Some(shortname)) {
            Some(idx) => {
                lastseen = Some(idx);
                let flag: &mut Flag = &mut flags[idx];

                /* +1 since we're trying to know if there are more characters (shortnames) to
                 * be parsed.
                 *
                 * srcoffset because we're skiping the '-' from the original `source` argument
                 */
                if flag.mode == ArgMode::Required && (i + 1 + srcoffset) < srclen {
                    return Err(Error::BadGrouping(source.clone(), i + srcoffset))
                }

                flag.seen = true;
            },
            None => {
                return Err(Error::UnknownShortname(source.clone(), i + srcoffset))
            }
        }
    }

    Ok(lastseen)
}

fn parse_longopt (source: &String, flags: &mut [Flag]) -> Result<Option<usize>, Error> {
    let lastseen: Option<usize>;
    let givename: String;
    let mut argument: Option<String> = None;

    let flagnameoffset: usize = 2;
    match source.find('=') {
        Some(idx) => {
            givename = source[flagnameoffset..idx].to_string();
            argument = Some(source[idx..].to_string());
        }
        None => {
            givename = source[flagnameoffset..].to_string();
        }
    }

    match flags.iter().position(|f| f.longname == Some(&givename)) {
        Some(idx) => {
            lastseen = Some(idx);
            let flag: &mut Flag = &mut flags[idx];

            if let Some(_arg) = argument {
                todo!();
            }
            flag.seen = true;
        }
        None => {
            return Err(Error::UnknownLongname(source.clone(), flagnameoffset, givename.len()))
        }
    }

    Ok(lastseen)
}

pub fn argrs (args: Vec<String>, flags: &mut [Flag]) -> Result<Argrs, Error> {
    check_integrity(flags) ?;
    let mut f_lasidx: Option<usize> = None;

    for arg in args.iter().skip(1) {
        let length: usize = arg.len();
        let char1 : Option<char> = arg.chars().nth(0);
        let char2 : Option<char> = arg.chars().nth(1);

        match (length, char1, char2) {
            (2.., Some('-'), Some(ch2)) if ch2.is_ascii_alphanumeric() => {
                f_lasidx = parse_shortopt(&arg, flags) ?;
            }
            (3.., Some('-'), Some('-')) => {
                f_lasidx = parse_longopt(&arg, flags) ?;
            }
            (2, Some('-'), Some('-')) => {
                todo!();
            }
            _ => {
                todo!();
            }
        }
    }

    // TODO: make sure the prev flag has its argument

    Ok(Argrs {
        flastindex: f_lasidx,
    })
}
