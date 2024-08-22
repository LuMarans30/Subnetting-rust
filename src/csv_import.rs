use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;

/**
 * Imports a CSV file <br>
 * Example:
 * ```csv
 * ip,cidr
 * number_of_hosts1
 * number_of_hosts2
 * ...
 * number_of_hostsN
 * ```
 */
pub fn import_csv(file_path: &str) -> Result<(String, u32, Vec<u32>), Box<dyn std::error::Error>> {
    let file = File::open(Path::new(file_path))?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    // Read and parse the first line
    let first_line = lines.next().ok_or("File is empty")??;

    let parts: Vec<&str> = first_line.split(&[',', '/'][..]).collect();
    let (ip, cidr) = match parts.as_slice() {
        [ip, cidr] => (ip.trim().to_string(), cidr.trim().parse()?),
        _ => return Err(format!("Invalid first line format: {}", first_line).into()),
    };

    let num_hosts_array: Vec<u32> = lines
        .map(|line| {
            let line = line?;
            line.trim().parse().map_err(|e: ParseIntError| e.into())
        })
        .collect::<Result<_, Box<dyn std::error::Error>>>()?;

    if num_hosts_array.is_empty() {
        return Err("No host numbers found in the file".into());
    }

    Ok((ip, cidr, num_hosts_array))
}
