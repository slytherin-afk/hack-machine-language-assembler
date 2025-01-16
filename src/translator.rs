fn handle_c_instruction(line: &String) -> Result<u16, String> {
    let mut dest_encoded = "000";
    let mut jump_encoded = "000";
    let mut comp = None;

    if line.contains("=") {
        let dest = line.split("=").nth(0).unwrap().trim();

        comp = Some(
            line.split("=")
                .nth(1)
                .unwrap()
                .split(";")
                .nth(0)
                .unwrap()
                .trim(),
        );

        dest_encoded = match dest {
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => return Err(format!("Invalid destination [{0}]", dest)),
        };
    };

    if line.contains(";") {
        let jump = line.split(";").last().unwrap().trim();

        comp = if let None = comp {
            Some(
                line.split(";")
                    .nth(0)
                    .unwrap()
                    .split('=')
                    .last()
                    .unwrap()
                    .trim(),
            )
        } else {
            comp
        };

        jump_encoded = match jump {
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => return Err(format!("Invalid jump [{0}]", jump)),
        };
    };

    comp = if let None = comp {
        Some(line.trim())
    } else {
        comp
    };

    let comp_encoded = match comp.unwrap() {
        "0" => b"0101010",
        "1" => b"0111111",
        "-1" => b"0111010",
        "D" => b"0001100",
        "A" => b"0110000",
        "!D" => b"0001101",
        "!A" => b"0110001",
        "-D" => b"0001111",
        "-A" => b"0110011",
        "D+1" => b"0111111",
        "A+1" => b"0110111",
        "D-1" => b"0001110",
        "A-1" => b"0110010",
        "D+A" => b"0000011",
        "D-A" => b"0010011",
        "A-D" => b"0000111",
        "D&A" => b"0000000",
        "D|A" => b"0010101",
        "M" => b"1110000",
        "!M" => b"1110001",
        "-M" => b"1110011",
        "M+1" => b"1110111",
        "M-1" => b"1110010",
        "D+M" => b"1000010",
        "D-M" => b"1010011",
        "M-D" => b"1000111",
        "D&M" => b"1000000",
        "D|M" => b"1010101",
        _ => return Err(format!("Invalid comp [{0}]", comp.unwrap())),
    }
    .iter()
    .map(|&c| c as char)
    .collect::<String>();

    let isc = format!("111{0}{1}{2}", comp_encoded, dest_encoded, jump_encoded);

    Ok(u16::from_str_radix(&isc, 2).unwrap())
}

fn handle_a_instruction(line: &String) -> u16 {
    let instruction = &line[1..];
    instruction.parse().unwrap()
}

pub fn translate_to_binary(lines: &Vec<String>) -> Result<Vec<u16>, String> {
    lines
        .iter()
        .map(|x| {
            if x.starts_with("@") {
                return Ok(handle_a_instruction(x));
            } else {
                return handle_c_instruction(x);
            }
        })
        .collect()
}
