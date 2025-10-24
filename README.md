# External Merge Sort for Large Files

[![Rust CI](https://github.com/bartossh/sorter/workflows/Rust%20CI/badge.svg)](https://github.com/bartossh/sorter/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://docs.rs/sort_bigger_then_ram/badge.svg)](https://docs.rs/sort_bigger_then_ram)

A high-performance Rust implementation of external merge sort designed to efficiently sort files that are larger than available RAM. This tool uses a divide-and-conquer approach to sort massive datasets by breaking them into manageable chunks, sorting each chunk in memory, and then merging the sorted chunks. The implementation is optimized for sorting large files containing unsigned 64-bit integers.

## Features

- **Memory-efficient**: Sorts files larger than available RAM by processing data in configurable batches
- **Fast performance**: Uses unstable sort for better performance when stability isn't required
- **Configurable batch size**: Adjust the number of elements processed in each batch
- **Clean temporary file management**: Automatically cleans up temporary files after sorting
- **Command-line interface**: Easy-to-use CLI with clear options
- **Error handling**: Robust error handling using the `anyhow` crate

## Algorithm

The implementation uses the external merge sort algorithm:

1. **Chunking Phase**: Reads the input file in batches (default 1024 elements)
2. **In-memory Sorting**: Each batch is sorted in memory using Rust's efficient unstable sort
3. **Temporary Storage**: Sorted batches are written to temporary files
4. **K-way Merge**: Uses a min-heap (BinaryHeap) to efficiently merge all sorted temporary files
5. **Cleanup**: Automatically removes temporary files after successful merge

### Time Complexity
- Sorting phase: O(n log n)
- Merge phase: O(n log k) where k is the number of temporary files
- Overall: O(n log n)

### Space Complexity
- Memory usage: O(batch_size * 8 bytes) for u64 values
- Disk usage: O(n) for temporary files

## Installation

### From Source

```bash
git clone https://github.com/bartossh/sorter.git
cd sorter
cargo build --release
```

The binary will be available at `target/release/sort_bigger_then_ram`.

> **Note**: The repository is named `sorter` but the crate/binary name is `sort_bigger_then_ram`.

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

## Usage

```bash
sort_bigger_then_ram --input <INPUT_FILE> --output <OUTPUT_FILE> [--batch <BATCH_SIZE>]
```

### Options

- `-i, --input <INPUT>`: Path to the input file containing numbers (one per line)
- `-o, --output <OUTPUT>`: Path where the sorted output will be written
- `-b, --batch <BATCH>`: Size of RAM in bytes to use in each batch (default: 1024). 
- `-h, --help`: Display help information
- `-V, --version`: Display version information

### Examples

Sort a large file with default settings:
```bash
sort_bigger_then_ram -i huge_numbers.txt -o sorted_numbers.txt
```

Sort with a smaller batch size (useful for systems with limited RAM):
```bash
sort_bigger_then_ram -i huge_numbers.txt -o sorted_numbers.txt -b 10000
```

Sort with a larger batch size (for better performance on systems with more RAM):
```bash
sort_bigger_then_ram -i huge_numbers.txt -o sorted_numbers.txt -b 10000000
```

## Input Format

The input file should contain unsigned 64-bit integers (u64), one per line:

```
42
1337
9999999999
123
456789
```

## Performance Considerations

1. **Batch Size**: Larger batch sizes generally improve performance but require more RAM
   - Default: 1024 elements (~8 KB for u64)
   - Each u64 takes 8 bytes, so batch_size * 8 = memory in bytes
   - Example: 1,000,000 elements = ~8 MB of RAM
   - Memory formula: `RAM usage ≈ (batch_size × 8 bytes) + overhead`
   - Recommended: Set batch size based on available RAM divided by 8

2. **Disk I/O**: Performance is heavily dependent on disk speed
   - SSD recommended for best performance
   - Ensure sufficient free disk space (at least 2x the input file size)

3. **Number Format**: Currently supports only unsigned 64-bit integers (u64)

## Implementation Details

### Key Components

- **Sorter struct**: Main implementation containing sort logic
- **Binary Heap**: Used for efficient k-way merge of sorted chunks with `Reverse` wrapper for min-heap behavior
- **Temporary Files**: Named with pattern `sort_temp_file_<number>.txt` in the same directory as the output file
- **Error Handling**: Uses `anyhow::Result` for comprehensive error propagation
- **Empty File Handling**: Properly handles empty input files by creating an empty output file

### Memory Usage

The tool uses approximately:
- RAM: batch_size * 8 bytes + overhead for heap and buffers
- Disk: Size of input file (for temporary files)

## Building and Testing

### Build
```bash
cargo build --release
```

### Run tests
```bash
cargo test
```

The project includes comprehensive integration tests that verify:
- Sorting of small and large files
- Handling of already sorted and reverse sorted data
- Proper handling of duplicates
- Edge cases like single numbers and empty files
- Temporary file cleanup
- Various batch sizes

### Run benchmarks
```bash
cargo bench
```

The project includes performance benchmarks using Criterion.rs to measure sorting performance across different file sizes and batch configurations. The benchmarks test:
- Small files (1K numbers)
- Medium files (100K numbers) with different batch sizes
- Large files (1M numbers)
- Impact of various batch sizes (100, 500, 1000, 5000, 10000 elements)

### Generate test data
```bash
# Generate a file with random numbers
seq 1 1000000 | sort -R > test_input.txt
```

## Limitations

- Currently only supports unsigned 64-bit integers (u64)
- Requires free disk space approximately equal to input file size
- Performance depends on disk I/O speed
- Temporary files are created in the same directory as the output file

## Future Improvements

- [ ] Support for different data types (signed integers, floats, strings)
- [ ] Parallel sorting of individual chunks
- [ ] Custom temporary file location
- [ ] Progress indicators for large files
- [ ] Support for custom separators
- [ ] Memory-mapped file support for better performance
- [ ] Compression of temporary files to reduce disk usage

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for command-line parsing
- Error handling powered by [anyhow](https://github.com/dtolnay/anyhow)
- Benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs) for performance measurement
- Tests use [serial_test](https://github.com/palfrey/serial_test) to ensure proper cleanup between tests
- Random number generation in benchmarks uses [rand](https://github.com/rust-random/rand)
- Inspired by the classic external merge sort algorithm used in database systems