use std::path::Path;

use clap::{App, Arg, SubCommand, ArgMatches};

use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() -> Result<(), std::io::Error>{
    let args =
        App::new("action-kv")
            .version("0.1")
            .usage(USAGE)
            .arg(Arg::with_name("filename")
                .takes_value(true)
                .required(true))
            .subcommands(vec![
                SubCommand::with_name("keys"),
                SubCommand::with_name("get")
                    .arg(Arg::with_name("key").takes_value(true).required(true)),
                SubCommand::with_name("delete")
                    .arg(Arg::with_name("key").takes_value(true).required(true)),
                SubCommand::with_name("insert")
                    .arg(Arg::with_name("key").takes_value(true).required(true))
                    .arg(Arg::with_name("value").takes_value(true).required(true)),
                SubCommand::with_name("update")
                    .arg(Arg::with_name("key").takes_value(true).required(true))
                    .arg(Arg::with_name("value").takes_value(true).required(true))
            ])
            .get_matches();


    let filename = args.value_of("filename").expect("filename is missing");

    let mut cmd: Option<(String, &ArgMatches)> = None;

    for name in &["get", "delete", "insert", "update"] {
        if let Some(matched) = args.subcommand_matches(name) {
            cmd = Some((String::from(*name), matched));
            break;
        }
    }

    let mut store = ActionKV::open(Path::new(filename))?;
    store.load()?;

    match cmd {
        None => println!("Key-value store size: {}", store.index.len()),
        Some((name, matched)) => {
            let key_string = matched.value_of("key").expect("key is missing");
            let key = key_string.as_ref();
            let maybe_value = matched.value_of("value");
            match name.as_ref() {
                "get" => match store.get(key)? {
                    None => eprintln!("{:?} not found", key_string),
                    Some(value) => println!("{:?}", String::from_utf8(value).ok().unwrap()),
                },
                "delete" => store.delete(key)?,
                "insert" => {
                    let value = maybe_value.expect(USAGE).as_ref();
                    store.insert(key, value)?;
                },
                "update" => {
                    let value = maybe_value.expect(USAGE).as_ref();
                    store.update(key, value)?;
                },
                _ => eprintln!("{}", &USAGE),
            }
        }
    };

    Ok(())
}