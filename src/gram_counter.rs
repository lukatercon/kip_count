use std::fs;
use std::io::{BufWriter, Write};
use std::collections::HashMap;
use std::path::Path;
use itertools::Itertools;

pub struct GramCounter {
    gram_count: HashMap<String, i64>,
    k: usize,
    n: usize,
    kn: usize,
}

impl GramCounter {
    pub fn new(k: usize, n: usize) -> GramCounter {
        GramCounter {
            gram_count: HashMap::new(),
            k,
            n,
            kn: k + n
        }
    }
    

    pub fn write_to_file(skipgrams_vec: Vec<(&String, &i64)>, output_path: &Path) {
        let curr_file = fs::File::create(output_path).unwrap();
        let mut file_writer = BufWriter::new(curr_file);

        for i in 0..skipgrams_vec.len() {
            let curr_line = format!("{}: {}", skipgrams_vec[i].0, skipgrams_vec[i].1);

            writeln!(file_writer, "{curr_line}").unwrap();
        }

        file_writer.flush().unwrap();
    }


    pub fn get_sorted_skipgrams(&self) -> Vec<(&String, &i64)> {
        let mut grams_sorted: Vec<(&String, &i64)> = self.gram_count.iter().collect();
        grams_sorted.sort_by(|a, b| b.1.cmp(&a.1));

        grams_sorted

    }


    fn update_counter(&mut self, gram_to_add: String) {
        match self.gram_count.get(&gram_to_add) {
                    Some(count) => self.gram_count.insert(gram_to_add.to_string(), *count + 1),
                    None              => self.gram_count.insert(gram_to_add.to_string(), 1),
                };
    }


    fn count_skipgrams(&mut self, contents: Vec<&str>) {
        for i in 0..contents.len() {
            // first check whether the current word is too close to the end to be the start of a skipgram
            if contents.len() - i < (self.kn) {
                break;
            }

            // get the current k+n-gram - this is not yet what we are looking for, 
            // but if we systematically remove k elements
            // in all possible combinations, then we get there!
            let curr_kngram: Vec<&str> = contents[i..(i + self.kn)].to_vec();

            // create a vector from a range of size k + n. 
            // This is the vec from which we will remove several elements.
            let kn_range: Vec<usize> = (0..(self.kn)).collect();

            let combinations_to_remove = kn_range.iter().combinations(self.k);

            // go through every combination of items to remove and replace said items with an "X"
            for comb in combinations_to_remove {
                let mut skipgram_to_add = curr_kngram.clone();
                for i in 0..(self.kn) {
                    if comb.contains(&&i) {
                        skipgram_to_add[i] = "X";
                        
                    }
                }

                let skipgram_to_add = skipgram_to_add.join(" ");

                self.update_counter(skipgram_to_add);

            }
        }

    }


    pub fn count_from_directory(&mut self, input_dir: &Path) {
        println!("Counting {}-skip-{}-grams in directory: {:?}", self.k, self.n, input_dir);

        // go through all the files in the directory
        for curr_file in fs::read_dir(input_dir).unwrap() {
            let curr_file_path = curr_file.unwrap().path();
            //println!("Reading file: {}", curr_file_path.display());

            let curr_file_contents = fs::read_to_string(curr_file_path).unwrap();

            // split the contents on spaces
            let contents_split = curr_file_contents.split_whitespace().collect::<Vec<&str>>();

            // count the skipgrams
            self.count_skipgrams(contents_split);

        }
    }
}