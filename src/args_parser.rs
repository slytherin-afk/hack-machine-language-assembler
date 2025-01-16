pub struct Arguments {
    pub input_file_path: String,
    pub output_file_path: String,
}

impl Arguments {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let mut output_file_path = None;
        let mut input_file_path = None;
        let mut iterator = args.into_iter();

        while let Some(arg) = iterator.next() {
            if arg == "-o" {
                output_file_path = iterator.next()
            } else {
                input_file_path = Some(arg);
            }
        }

        Ok(Arguments {
            input_file_path: input_file_path.ok_or("Input file not given in args")?,
            output_file_path: output_file_path.ok_or("Output file not given in args")?,
        })
    }
}
