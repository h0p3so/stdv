use std::collections::HashMap;
use std;

#[derive(Debug)]
pub enum Error {
    InvalidShortname,
    DuplicatedShortname,
    DuplicatedLongname,
    MalformedFlag,

    UnknownShortname,
    UnknownLongname,
    NonsenseArgument,
    WrongTypeProvided,
}

pub enum FValue {
    Txt(String),
    F64(f64),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64)
}

impl FValue {
    pub fn as_i32 (&self) -> Option<i32> {
        if let FValue::I32(i) = *self {
            Some(i)
        }
        else {
            None
        }
    }
}

#[derive(PartialEq)]
pub enum FExpectedType {
    Txt,
    F64,
    I32,
    U32,
    I64,
    U64
}

#[derive(PartialEq)]
pub enum FArgMode {
    Required,
    Optional,
    NoArgument
}

pub struct Flag {
    pub shortname: char,
    pub longname: Option<&'static str>,
    pub mode: FArgMode,
    pub seen: bool,
    pub value: Option<FValue>,
    pub extype: Option<FExpectedType>,

    position: usize
}

impl Flag {
    fn default () -> Self {
        Self {
            shortname: '\0',
            longname: None,
            seen: false,
            value: None,
            extype: None,
            mode: FArgMode::NoArgument,
            position: 0
        }
    }
}

pub fn grs_parse (args: Vec<String>, flags: &mut [Flag]) -> Result<Vec<String>, Error> {
    check_flags(flags) ?;

    let mut positionalargs: Vec<String> = Vec::new();
    let mut is_pos_arg: bool = false;
    let mut lastflagindex: Option<usize> = None;

    for arg in args {
        if is_pos_arg {
            positionalargs.push(arg);
            continue;
        }

        let first_ch: char  = arg.chars().nth(0).unwrap_or('\0');
        let second_ch: char = arg.chars().nth(1).unwrap_or('\0');
        let length: usize   = arg.len();

        if first_ch == '-' && second_ch.is_ascii_alphanumeric() && length >= 2 {
            lastflagindex = parse_shortopt(&arg, flags) ?;
        }
        else if arg.starts_with("--") && length >= 3 {
            lastflagindex = parse_longopt(&arg, flags) ?;
        }
        else if arg == "--" {
            is_pos_arg = true;
        }
        else {
            if lastflagindex.is_none() {
                return Err(Error::NonsenseArgument)
            }

            parse_flag_argument(&arg, &mut flags[lastflagindex.unwrap()]) ?;
            lastflagindex = None;
        }
    }

    Ok(positionalargs)
}

fn parse_flag_argument (source: &String, flag: &mut Flag) -> Result<(), Error> {
    if flag.mode == FArgMode::NoArgument {
        return Err(Error::NonsenseArgument);
    }

    match flag.extype.as_ref().unwrap() {
        FExpectedType::Txt => flag.value = Some(FValue::Txt(source.to_string())),
        FExpectedType::F64 => flag.value = Some(FValue::F64(source.parse::<f64>().map_err(|_| Error::WrongTypeProvided)?)),
        FExpectedType::I32 => flag.value = Some(FValue::I32(source.parse::<i32>().map_err(|_| Error::WrongTypeProvided)?)),
        FExpectedType::U32 => flag.value = Some(FValue::U32(source.parse::<u32>().map_err(|_| Error::WrongTypeProvided)?)),
        FExpectedType::I64 => flag.value = Some(FValue::I64(source.parse::<i64>().map_err(|_| Error::WrongTypeProvided)?)),
        FExpectedType::U64 => flag.value = Some(FValue::U64(source.parse::<u64>().map_err(|_| Error::WrongTypeProvided)?)),
        _ => todo!(),
    }

    Ok(())
}

fn parse_longopt (source: &String, flags: &mut [Flag]) -> Result<Option<usize>, Error> {
    let mut flagpos: Option<usize> = None;
    let mut name: String = String::new();
    let mut argument: Option<String> = None;

    match source.find('=') {
        Some(index) => {
            name = source[2..index].to_string();
            argument = Some(source[index..].to_string());
        },
        None => {
            name = source[2..].to_string();
        }
    };

    match flags.iter_mut().find(|f| !f.longname.is_none() && f.longname.unwrap() == name) {
        Some(f) => {
            f.seen = true;
            flagpos = Some(f.position);
        },
        None => return Err(Error::UnknownLongname)
    }

    Ok(flagpos)
}

fn parse_shortopt (source: &String, flags: &mut [Flag]) -> Result<Option<usize>, Error> {
    let mut flagpos: Option<usize> = None;
    for shortname in source.chars().skip(1) {
        match flags.iter_mut().find(|f| f.shortname == shortname) {
            Some(f) => {
                f.seen = true;
                flagpos = Some(f.position);
            },
            None => return Err(Error::UnknownShortname)
        }
    }

    Ok(flagpos)
}

fn check_flags (flags: &mut [Flag]) -> Result<(), Error> {
    let mut mapper: HashMap<char, bool> = HashMap::new();

    for (i, flag) in flags.iter_mut().enumerate() {
        if !flag.shortname.is_ascii_alphanumeric() {
            return Err(Error::InvalidShortname)
        }
        if mapper.contains_key(&flag.shortname) {
            return Err(Error::DuplicatedShortname)
        }

        if flag.mode == FArgMode::NoArgument && !flag.extype.is_none() {
            return Err(Error::MalformedFlag)
        }

        flag.position = i;
        mapper.insert(flag.shortname, true);
    }

    for i in 0..flags.len() {
        let longname: Option<&str> = flags[i].longname;
        if longname.is_none() {
            continue;
        }

        for j in (i + 1)..flags.len() {
            let jname: Option<&str> = flags[j].longname;
            if !jname.is_none() && Some(jname) == Some(longname) {
                return Err(Error::DuplicatedLongname)
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works () {
        let args: Vec<String> = vec!["654".to_string(), "54".to_string()];
        let mut flags: [Flag; 2] = [
            Flag {
                shortname: 'v',
                longname: Some("verbose"),
                ..Flag::default()
            },
            Flag {
                shortname: 'f',
                longname: Some("file"),
                mode: FArgMode::Required,
                extype: Some(FExpectedType::I32),
                ..Flag::default()
            }
        ];

        println!("Error code: {:?}", grs_parse(args, &mut flags));
        assert_eq!(1, 1);
    }
}
