use std::path::Path;

pub struct Config<'a> {
    k: usize,
    n: usize,
    input_dir: &'a Path,
    output_file: &'a Path,
}

impl<'a> Config<'a> {
    pub fn parse_args(args: &'a Vec<String>) -> Config<'a> {
        Config { 
            k: args[1].parse::<usize>().unwrap(),
            n: args[2].parse::<usize>().unwrap(),
            input_dir: Path::new(&args[3]),
            output_file: Path::new(&args[4])
        }
    }

    pub fn get_k(&self) -> usize {
        self.k
    }

    pub fn get_n(&self) -> usize {
        self.n
    }

    pub fn get_input_dir(&self) -> &Path {
        self.input_dir
    }

    pub fn get_output_file(&self) -> &Path {
        self.output_file
    }

    pub fn check_validity(&self) {
        // check whether n is at least k + 1 or larger
        if self.n < self.k + 1 {
            panic!("n must be at least k + 1 or larger. Please rerun with different parameters")
        }
    }
}