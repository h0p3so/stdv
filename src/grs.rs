use std::collections::HashMap;

pub enum ParsingError {
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

#[derive(PartialEq)]
pub enum ArgMode {
    NoArgument,
    Optional,
    Required
}

pub enum ArgExpectedType {
    Text,
    Float,
    U32,
    I32,
    U64,
    I64
}

pub enum FlagValue {
    Text(String),
    Float(f64),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64)
}

pub struct Flag {
    pub shortname: char,
    pub longname: Option<&'static str>,
    pub mode: ArgMode,
    pub value: Option<FlagValue>,
    pub seen: bool,
    pub expected_type: Option<ArgExpectedType>
}

pub struct ParsedResult {
    pub positional: Vec<String>,
}

impl ParsingError {
    pub fn is_programmer_fault (&self) -> bool {
        match self {
            ParsingError::InvalidShortname
                | ParsingError::DuplicatedShortname
                | ParsingError::DuplicatedLongname
                | ParsingError::MalformedOpts => true,
            _ => false
        }
    }

    pub fn is_user_fault (&self) -> bool {
        !self.is_programmer_fault()
    }
}

pub fn grs_init (args: &[&str], flags: &mut [Flag], callername: &str) -> Result<ParsedResult, ParsingError> {
    check_integrity(flags) ?;

    let mut posargs: Vec<String> = Vec::new();
    let mut onlyargs: bool = false;

    for arg in args {
        if onlyargs {
            posargs.push(arg.to_string());
            continue;
        }

        let stc : Option<char> = arg.chars().nth(0);
        let ndc : Option<char> = arg.chars().nth(1);
        let alen: usize = arg.len();

        if stc == Some('-') && matches!(ndc, Some(c) if c.is_ascii_alphanumeric()) {
            parse_shortopt(flags, arg) ?;
        }
        else if arg.starts_with("--") && alen > 2 {
            parse_longopt(flags, arg) ?;
        }
        else if arg.starts_with("--") && alen == 2 {
            onlyargs = true;
            continue;
        }
        else {
            todo!()
        }
    }

    return Ok(ParsedResult{ positional: posargs })
}

fn check_integrity (flags: &mut [Flag]) -> Result<(), ParsingError> {
    let mut mapper: HashMap<char, bool> = HashMap::new();

    for (i, flag) in flags.iter_mut().enumerate() {
        if mapper.contains_key(&flag.shortname) {
            return Err(ParsingError::DuplicatedShortname);
        }
        if !flag.shortname.is_ascii_alphanumeric() {
            return Err(ParsingError::InvalidShortname);
        }

        mapper.insert(flag.shortname, true);
        flag.seen = false;
        flag.value = None;
    }

    let nmemb: usize = flags.len();
    for i in 0..nmemb {
        let longname = match flags[i].longname {
            Some(n) => n,
            None    => continue
        };
        for j in (i + 1)..nmemb {
            if flags[j].longname == Some(longname) {
                return Err(ParsingError::DuplicatedLongname);
            }
        }
    }


    return Ok(())
}

fn parse_shortopt (flags: &mut [Flag], src: &str) -> Result<(), ParsingError> {
    for c in src.chars().skip(1) {
        let flag = flags.iter_mut().find(|f| f.shortname == c);
        match flag {
            Some(f) => f.seen = true,
            None => return Err(ParsingError::UnknownShortname)
        };
    }

    Ok(())
}

fn parse_longopt (flags: &mut[Flag], src: &str) -> Result<(), ParsingError> {
    let eqsign: Option<usize> = src.find('=');

    let (longname, argument) = match eqsign {
        Some(i)	=> (&src[2..i], Some(&src[(i + 1)..])),
        None => (&src[2..], None),
    };

    match flags.iter_mut().find(|f| f.longname == Some(longname)) {
        Some(f) => f.seen = true,
        None => { return Err(ParsingError::UnknownLongname); }
    };

    Ok(())
}

fn parse_argument (flag: &mut Flag, src: &str) -> Result<(), ParsingError> {
    if flag.mode == ArgMode::NoArgument {
        return Err(ParsingError::NonsenseArgument);
    }

    let expected = match &flag.expected_type {
        Some(t) => t,
        None => return Err(ParsingError::MalformedOpts),
    }

    match flag.expected_type {
        ArgExpectedType::Text  => flag.value = Some(FlagValue::Text(src.to_string())),
        ArgExpectedType::Float => flag.value = Some(FlagValue::Float(src.parse::<f64>().map_err(|_| ParsingError::MalformedOpts))?),
        ArgExpectedType::U32   => flag.value = Some(FlagValue::U32(src.parse::<u32>().map_err(|_| ParsingError::MalformedOpts))  ?),
        ArgExpectedType::I32   => flag.value = Some(FlagValue::I32(src.parse::<i32>().map_err(|_| ParsingError::MalformedOpts))  ?),
        ArgExpectedType::I64   => flag.value = Some(FlagValue::I64(src.parse::<i64>().map_err(|_| ParsingError::MalformedOpts))  ?),
        ArgExpectedType::U64   => flag.value = Some(FlagValue::U64(src.parse::<u64>().map_err(|_| ParsingError::MalformedOpts))  ?),
    }

    Ok(())
}

