use std::collections::HashMap;
use std;

#[derive(Debug)]
pub enum Error {
    InvalidShortname,
    DuplicatedShortname,
    DuplicatedLongname,

    UnknownShortname
}

pub enum FValue {
    Text(String),
    F64(f64),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64)
}

pub enum FExpectedType {
    Text,
    F64,
    I32,
    U32,
    I64,
    U64
}

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
    let mut lastflag_index: usize = 0;

    for arg in args {
        if is_pos_arg {
            positionalargs.push(arg);
            continue;
        }

        let first_ch: char  = arg.chars().nth(0).unwrap_or('\0');
        let second_ch: char = arg.chars().nth(1).unwrap_or('\0');
        let length: usize   = arg.len();

        if first_ch == '-' && second_ch.is_ascii_alphanumeric() && length >= 2 {
            lastflag_index = parse_shortopt(&arg, flags) ?;
        }
    }

    Ok(positionalargs)
}

fn parse_shortopt (source: &String, flags: &mut [Flag]) -> Result<usize, Error> {
    let mut flagpos: usize = 0;
    for shortname in source.chars() {
        let flag = flags.iter_mut().find(|f| f.shortname == shortname);
        if flag.is_none() {
            return Err(Error::UnknownShortname)
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
        let args: Vec<String> = vec!["-g".to_string()];
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
                extype: Some(FExpectedType::Text),
                ..Flag::default()
            }
        ];

        println!("Error code: {:?}", grs_parse(args, &mut flags));
        assert_eq!(1, 1);
    }
}
