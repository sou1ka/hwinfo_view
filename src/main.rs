use winreg::enums::*;
use winreg::RegKey;
use std::collections::HashMap;
use std::vec::Vec;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hwinfo = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags("SOFTWARE\\HWiNFO64\\VSB", KEY_READ)?;
    let enums = hwinfo.enum_values();
    let mut list: Vec<HashMap<String, String>> = Vec::new();
    let mut count = 0;
    let mut map: HashMap<String, String> = HashMap::new();

    for (name, value) in enums.map(|x| x.unwrap()) {
        let reg = Regex::new(r"\d+").unwrap();
        let cap = reg.captures(&name).unwrap();
        let num: usize = cap[0].parse().unwrap();
        let nm = name.replace(&num.to_string(), "");

        if count != num {
            count = num;
            list.push(map.clone());
        }

        map.insert(nm, value.to_string());
    }

    list.push(map.clone()); // last

    println!("{:?}", list);

    Ok(())
}