use std::{collections::HashMap, str::Lines};

pub fn remove_comments_blanks<'a>(lines: Lines<'a>) -> Vec<&'a str> {
    lines
        .into_iter()
        .map(|x| x.split("//").nth(0).unwrap().trim())
        .filter(|x| x.len() > 0)
        .collect()
}

pub fn get_symbols<'a>(lines: &Vec<&'a str>) -> Result<HashMap<&'a str, String>, String> {
    let mut symbols = HashMap::new();
    let mut isc_number = 0;

    for (i, line) in lines.into_iter().enumerate() {
        if !line.starts_with("(") {
            isc_number += 1;
            continue;
        }

        if line.starts_with("(") && !line.ends_with(")") {
            return Err(format!("Invalid symbol exists [{0}]", line));
        }

        let mut j = 0;

        while let Some(next) = lines.get(i + j) {
            j += 1;

            if next.starts_with("(") {
                continue;
            }

            if ["@", "0", "D", "M", "A", "-1", "1", "!", "-"]
                .iter()
                .any(|&prefix| next.starts_with(prefix))
            {
                let key = &line[1..line.len() - 1];

                if symbols.contains_key(key) {
                    return Err(format!("Duplicate symbol exists [{0}]", line));
                }

                symbols.insert(key, isc_number.to_string());

                break;
            } else {
                return Err(format!("Invalid instructions exists [{0}]", next));
            }
        }
    }

    Ok(symbols)
}

pub fn get_variables<'a>(
    lines: &Vec<&'a str>,
    symbols: &HashMap<&'a str, String>,
) -> Result<HashMap<&'a str, String>, String> {
    let mut variables = HashMap::new();
    variables.insert("R0", "0".to_string());
    variables.insert("R1", "1".to_string());
    variables.insert("R2", "2".to_string());
    variables.insert("R3", "3".to_string());
    variables.insert("R4", "4".to_string());
    variables.insert("R5", "5".to_string());
    variables.insert("R6", "6".to_string());
    variables.insert("R7", "7".to_string());
    variables.insert("R8", "8".to_string());
    variables.insert("R9", "9".to_string());
    variables.insert("R10", "10".to_string());
    variables.insert("R11", "11".to_string());
    variables.insert("R12", "12".to_string());
    variables.insert("R13", "13".to_string());
    variables.insert("R14", "14".to_string());
    variables.insert("R15", "15".to_string());
    variables.insert("SCREEN", "16384".to_string());
    variables.insert("KBD", "24576".to_string());
    variables.insert("SP", "0".to_string());
    variables.insert("LCL", "1".to_string());
    variables.insert("ARG", "2".to_string());
    variables.insert("THIS", "3".to_string());
    variables.insert("THAT", "4".to_string());

    let mut start = 16;

    for line in lines.into_iter() {
        if line.starts_with("@") {
            let error = format!("Invalid variables exists [{0}]", line);
            let name: &str = line.split("@").nth(1).ok_or(error.clone())?;

            if name.parse::<u32>().is_ok() {
                continue;
            }

            if !symbols.contains_key(name) && !variables.contains_key(name) {
                variables.insert(name, start.to_string());
                start += 1;
            }
        }
    }

    Ok(variables)
}

pub fn translate_symbols_variables<'a>(
    lines: &Vec<&'a str>,
    symbols: &HashMap<&'a str, String>,
    variables: &HashMap<&'a str, String>,
) -> Vec<String> {
    lines
        .iter()
        .filter(|x| !x.starts_with("("))
        .map(|x| {
            if x.starts_with("@") {
                let value = x.split("@").nth(1).unwrap();
                if let Some(v) = symbols.get(value) {
                    return format!("@{}", v);
                }

                if let Some(v) = variables.get(value) {
                    return format!("@{}", v);
                }
            }
            x.to_string()
        })
        .collect()
}
