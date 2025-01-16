mod args_parser;
mod cleaner;
mod translator;

use std::{env, fs, io::Write};

fn main() {
    let args = args_parser::Arguments::build(env::args()).unwrap();
    let input_file = fs::read_to_string(&args.input_file_path).unwrap();
    let input_without_comments = cleaner::remove_comments_blanks(input_file.lines());
    let symbols = cleaner::get_symbols(&input_without_comments).unwrap();
    let variables = cleaner::get_variables(&input_without_comments, &symbols).unwrap();
    let mid_representation =
        cleaner::translate_symbols_variables(&input_without_comments, &symbols, &variables);
    let binary = translator::translate_to_binary(&mid_representation).unwrap();
    let mut output_file = fs::File::create(&args.output_file_path).unwrap();
    let data: String = binary.iter().map(|x| format!("{:016b}\n", x)).collect();

    output_file.write_all(data.as_bytes()).unwrap();
}
