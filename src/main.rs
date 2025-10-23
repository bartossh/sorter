use anyhow::Result;
use clap::Parser;
use std::cmp::Reverse;
use std::{
    collections::BinaryHeap,
    io::{BufRead, BufReader, Write},
};

const TEMP_FILE_PREFIX: &str = "./sort_temp_file_";

/// Sort a file that is bigger than RAM
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Sorter {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Output file path
    #[arg(short, long)]
    output: String,

    /// Batch size in megabytes, default 1024 MB
    #[arg(short, long, default_value_t = 1024)]
    batch: usize,
}

impl Sorter {
    fn sort_and_write_to_file(&self, batch_sorted: &mut [u64], next_num: u64) -> Result<()> {
        batch_sorted.sort_unstable();
        let mut file = std::fs::File::create(format!("{}{}.txt", TEMP_FILE_PREFIX, next_num))?;
        for num in batch_sorted {
            writeln!(file, "{}", num)?;
        }

        Ok(())
    }

    fn merge_files(&self, files_num: u64) -> Result<()> {
        let mut readers = (0..files_num)
            .map(|next_num| {
                BufReader::new(
                    std::fs::File::open(format!("{}{}.txt", TEMP_FILE_PREFIX, next_num)).unwrap(),
                )
                .lines()
            })
            .collect::<Vec<_>>();

        let mut heap = BinaryHeap::new();
        for (idx, reader) in readers.iter_mut().enumerate() {
            if let Some(Ok(line)) = reader.next() {
                let num: u64 = line.parse()?;
                heap.push(Reverse((num, idx)));
            }
        }

        let mut output_file = std::fs::File::create(&self.output)?;

        while let Some(item) = heap.pop() {
            let (num, idx) = item.0;
            writeln!(output_file, "{}", num)?;

            if let Some(Ok(line)) = readers[idx].next() {
                let next_num = line.parse()?;
                heap.push(Reverse((next_num, idx)));
            }
        }

        for file_num in (0..files_num).rev() {
            std::fs::remove_file(format!("{}{}.txt", TEMP_FILE_PREFIX, file_num))?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let sorter = Sorter::parse();

    let file = std::fs::File::open(&sorter.input)?;

    let reader = BufReader::new(file);
    let mut batch_sorted = Vec::with_capacity(sorter.batch);
    let mut file_counter = 0;
    for line_result in reader.lines() {
        let line = line_result?;

        let num: u64 = line.parse()?;
        batch_sorted.push(num);

        if batch_sorted.len() >= sorter.batch {
            sorter.sort_and_write_to_file(&mut batch_sorted, file_counter)?;
            file_counter += 1;
            batch_sorted.clear();
        }
    }
    if batch_sorted.len() >= sorter.batch {
        sorter.sort_and_write_to_file(&mut batch_sorted, file_counter)?;
    }

    sorter.merge_files(file_counter)?;

    Ok(())
}
