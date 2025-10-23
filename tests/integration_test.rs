use serial_test::serial;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

const TEST_DIR: &str = "./test_output";

fn setup_test_dir() {
    cleanup_test_dir();
    fs::create_dir_all(TEST_DIR).expect("Failed to create test directory");
}

fn cleanup_test_dir() {
    if Path::new(TEST_DIR).exists() {
        fs::remove_dir_all(TEST_DIR).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to cleanup test directory: {}", e);
        });
    }
}

fn cleanup_temp_files() {
    let dirs_to_clean = vec![
        std::env::current_dir().unwrap(),
        Path::new(TEST_DIR).to_path_buf(),
    ];

    for dir in dirs_to_clean {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    if filename.starts_with("sort_temp_file_") {
                        fs::remove_file(&path).unwrap_or_else(|e| {
                            eprintln!("Warning: Failed to remove temp file {:?}: {}", path, e);
                        });
                    }
                }
            }
        }
    }
}

fn create_test_file(filename: &str, numbers: &[u64]) -> String {
    fs::create_dir_all(TEST_DIR).expect("Failed to create test directory");

    let filepath = format!("{}/{}", TEST_DIR, filename);
    let mut file = File::create(&filepath).expect("Failed to create test file");

    for num in numbers {
        writeln!(file, "{}", num).expect("Failed to write to test file");
    }

    filepath
}

fn read_numbers_from_file(filepath: &str) -> Vec<u64> {
    let file = match File::open(filepath) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| line.parse::<u64>().ok())
        .collect()
}

fn run_sorter(input: &str, output: &str, batch_size: Option<usize>) -> Result<bool, String> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("-i")
        .arg(input)
        .arg("-o")
        .arg(output);

    if let Some(batch) = batch_size {
        cmd.arg("-b").arg(batch.to_string());
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Command failed with exit code: {:?}\nstdout: {}\nstderr: {}",
            output.status.code(),
            stdout,
            stderr
        ));
    }

    Ok(output.status.success())
}

#[test]
#[serial]
fn test_small_file_sort() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers = vec![42, 1337, 9999, 123, 456789, 1, 999999999];
    let input_file = create_test_file("small_input.txt", &numbers);
    let output_file = format!("{}/small_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, None) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    let mut expected = numbers.clone();
    expected.sort();

    assert_eq!(sorted_numbers, expected);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_already_sorted_file() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input_file = create_test_file("sorted_input.txt", &numbers);
    let output_file = format!("{}/sorted_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, None) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    assert_eq!(sorted_numbers, numbers);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_reverse_sorted_file() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let input_file = create_test_file("reverse_input.txt", &numbers);
    let output_file = format!("{}/reverse_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, None) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(sorted_numbers, expected);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_duplicates() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers = vec![5, 3, 5, 1, 3, 5, 1, 1, 3];
    let input_file = create_test_file("duplicates_input.txt", &numbers);
    let output_file = format!("{}/duplicates_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, None) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    let expected = vec![1, 1, 1, 3, 3, 3, 5, 5, 5];
    assert_eq!(sorted_numbers, expected);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_single_number() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers = vec![42];
    let input_file = create_test_file("single_input.txt", &numbers);
    let output_file = format!("{}/single_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, None) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    assert_eq!(sorted_numbers, numbers);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_small_batch_size() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers: Vec<u64> = (1..=100).rev().collect();
    let input_file = create_test_file("batch_input.txt", &numbers);
    let output_file = format!("{}/batch_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, Some(10)) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    let expected: Vec<u64> = (1..=100).collect();
    assert_eq!(sorted_numbers, expected);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_large_numbers() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers = vec![u64::MAX, u64::MAX - 1, u64::MAX - 2, 0, 1, u64::MAX / 2];
    let input_file = create_test_file("large_numbers_input.txt", &numbers);
    let output_file = format!("{}/large_numbers_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, None) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    let mut expected = numbers.clone();
    expected.sort();
    assert_eq!(sorted_numbers, expected);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_empty_file() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers: Vec<u64> = vec![];
    let input_file = create_test_file("empty_input.txt", &numbers);
    let output_file = format!("{}/empty_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, None) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let sorted_numbers = read_numbers_from_file(&output_file);
    assert_eq!(sorted_numbers, numbers);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
#[ignore] // This test requires a large amount of disk space
fn test_large_file() {
    setup_test_dir();
    cleanup_temp_files();

    let input_file = format!("{}/large_input.txt", TEST_DIR);
    let output_file = format!("{}/large_output.txt", TEST_DIR);

    let mut file = File::create(&input_file).expect("Failed to create large test file");
    for i in (0..10_000_000).rev() {
        writeln!(file, "{}", i).expect("Failed to write to large test file");
    }
    drop(file);

    match run_sorter(&input_file, &output_file, Some(100_000_000)) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    let file = File::open(&output_file).expect("Failed to open output file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map_while(Result::ok);

    let first_num: u64 = lines
        .next()
        .expect("Should have first line")
        .parse()
        .expect("Should parse first number");

    let mut last_num = first_num;
    for line in lines {
        if let Ok(num) = line.parse::<u64>() {
            last_num = num;
        }
    }

    assert_eq!(first_num, 0);
    assert_eq!(last_num, 9_999_999);

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_invalid_input_file() {
    setup_test_dir();
    cleanup_temp_files();

    let input_file = format!("{}/nonexistent.txt", TEST_DIR);
    let output_file = format!("{}/output.txt", TEST_DIR);

    if let Ok(success) = run_sorter(&input_file, &output_file, None) {
        assert!(!success, "Sorter should fail with nonexistent file")
    }

    cleanup_temp_files();
    cleanup_test_dir();
}

#[test]
#[serial]
fn test_temp_file_cleanup() {
    setup_test_dir();
    cleanup_temp_files();

    let numbers: Vec<u64> = (1..=1000).rev().collect();
    let input_file = create_test_file("cleanup_input.txt", &numbers);
    let output_file = format!("{}/cleanup_output.txt", TEST_DIR);

    match run_sorter(&input_file, &output_file, Some(50)) {
        Ok(success) => assert!(success, "Sorter should succeed"),
        Err(e) => panic!("Failed to run sorter: {}", e),
    }

    std::thread::sleep(std::time::Duration::from_millis(100));

    let temp_files: Vec<_> = fs::read_dir(TEST_DIR)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let filename = path.file_name()?.to_str()?;
            if filename.starts_with("sort_temp_file_") {
                Some(filename.to_string())
            } else {
                None
            }
        })
        .collect();

    assert!(
        temp_files.is_empty(),
        "Temp files not cleaned up: {:?}",
        temp_files
    );

    cleanup_temp_files();
    cleanup_test_dir();
}
