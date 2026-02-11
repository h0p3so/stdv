use std::collections::HashMap;
use std;

#[derive(Debug)]
pub enum Error {
    InvalidShortname,
    DuplicatedShortname,
    DuplicatedLongname
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
}

impl Flag {
    fn default () -> Self {
        Self {
            shortname: '\0',
            longname: None,
            seen: false,
            value: None,
            extype: None,
            mode: FArgMode::NoArgument
        }
    }
}

pub fn grs_parse (args: Vec<String>, flags: &mut [Flag]) -> Result<(), Error> {
    check_flags(flags) ?;

    Ok(())
}

fn check_flags (flags: &mut [Flag]) -> Result<(), Error> {
    let mut mapper: HashMap<char, bool> = HashMap::new();

    for flag in flags.iter_mut() {
        if !flag.shortname.is_ascii_alphanumeric() {
            return Err(Error::InvalidShortname)
        }
        if mapper.contains_key(&flag.shortname) {
            return Err(Error::DuplicatedShortname)
        }

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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works () {
        let args: Vec<String> = vec!["-v".to_string()];
        let mut flags: [Flag; 2] = [
            Flag {
                shortname: '&',
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

        println!("{:?}", grs_parse(args, &mut flags));
        assert_eq!(1, 1);
    }
}
