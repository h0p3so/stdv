use std::collections::HashSet;

#[derive(Debug)]
pub enum Error {
    /* usize: represents the index of the flag at fault */
    DupShortname(usize),
    DupLongname(usize),
    AnonymousFlag(usize),
    InvalidShortname(usize),
    ShouldntExpectedAValue(usize),
    ShouldExpectedAValue(usize),

    /* String: argument provided
     * usize: position of the shortopt at fault
     */
    UnknownShortname(String, usize),
    /* String: argument provided
     * usize: name's offset (always 2 '--')
     * usize: length of the flag name provided
     */
    UnknownLongname(String, usize, usize),
    /* String: argument provided
     * usize: position of the shortopt at fault
     */
    BadGrouping(String, usize),
    /* String: argument provided */
    PrematureArgument(String),
    /* String: argument provided
     * usize: index of the last flag seen
     */
    UnexpectedArgument(String, usize),
    WrongTypeProvided(String, usize)
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
    pub positional: Option<Vec<String>>,
    pub flastindex: Option<usize>, // XXX: may it is not needed (?)
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
            argument = Some(source[(idx + 1)..].to_string());
        }
        None => {
            givename = source[flagnameoffset..].to_string();
        }
    }

    match flags.iter().position(|f| f.longname == Some(&givename)) {
        Some(idx) => {
            lastseen = Some(idx);
            flags[idx].seen = true;
        }
        None => {
            return Err(Error::UnknownLongname(source.clone(), flagnameoffset, givename.len()))
        }
    }

    if let Some(arg) = argument {
        parse_argument(&arg.clone(), lastseen, flags) ?;
    }
    Ok(lastseen)
}

fn parse_argument (source: &String, flastindex: Option<usize>, flags: &mut [Flag]) -> Result<(), Error> {
    if flastindex.is_none() {
        return Err(Error::PrematureArgument(source.clone()));
    }

    let index: usize = flastindex.unwrap();
    let flag: &mut Flag = &mut flags[index];

    if flag.mode == ArgMode::NoArgument || flag.value.is_some() {
        return Err(Error::UnexpectedArgument(source.clone(), index));
    }

    /* it is guaranteed that there's an expected type thank to 'check_integrity'
     * function
     */
    match flag.expected.unwrap() {
        ArgExpectedType::Txt => flag.value = Some(ArgValue::Txt(source.clone())),
        ArgExpectedType::I32 => flag.value = Some(ArgValue::I32(
            source.parse::<i32>().map_err(|_| Error::WrongTypeProvided(source.clone(), index))?
        )),
        ArgExpectedType::U32 => flag.value = Some(ArgValue::U32(
            source.parse::<u32>().map_err(|_| Error::WrongTypeProvided(source.clone(), index))?
        )),
        ArgExpectedType::I64 => flag.value = Some(ArgValue::I64(
            source.parse::<i64>().map_err(|_| Error::WrongTypeProvided(source.clone(), index))?
        )),
        ArgExpectedType::U64 => flag.value = Some(ArgValue::U64(
            source.parse::<u64>().map_err(|_| Error::WrongTypeProvided(source.clone(), index))?
        )),
        ArgExpectedType::F64 => flag.value = Some(ArgValue::F64(
            source.parse::<f64>().map_err(|_| Error::WrongTypeProvided(source.clone(), index))?
        )),
    }
    Ok(())
}

pub fn argrs (args: Vec<String>, flags: &mut [Flag]) -> Result<Argrs, Error> {
    check_integrity(flags) ?;

    let mut flastindex: Option<usize> = None;

    for (i, arg) in args.iter().skip(1).enumerate() {
        let length: usize = arg.len();
        let char1 : Option<char> = arg.chars().nth(0);
        let char2 : Option<char> = arg.chars().nth(1);

        match (length, char1, char2) {
            (2.., Some('-'), Some(ch2)) if ch2.is_ascii_alphanumeric() => {
                flastindex = parse_shortopt(&arg, flags) ?;
            }
            (3.., Some('-'), Some('-')) => {
                flastindex = parse_longopt(&arg, flags) ?;
            }
            (2, Some('-'), Some('-')) => {
                return Ok( Argrs {
                    positional: Some(args[(i + 2)..].iter().cloned().collect()),
                    flastindex
                });
            }
            _ => {
                parse_argument(&arg, flastindex, flags) ?;
            }
        }
    }

    // TODO: make sure the prev flag has its argument

    Ok(Argrs {
        positional: None,
        flastindex,
    })
}
