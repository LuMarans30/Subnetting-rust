use std::io::{self, Write};

mod csv_import;
mod save_file;
mod subnet;
mod subnets_calculator;

use csv_import::import_csv;
use save_file::SaveToFile;
use subnet::SubnetError;
use subnets_calculator::SubnetCalculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Choose an option:");
    println!("1. Enter network information manually");
    println!("2. Import from CSV file");

    print!("Enter your choice: ");
    let choice = get_input()?;

    let (ip, cidr, num_hosts_array) = match choice.as_str() {
        "1" => {
            let (ip, cidr) = get_network_input()?;
            let num_subnets = get_num_subnets()?;
            let num_hosts_array = get_num_hosts(num_subnets)?;
            (ip, cidr, num_hosts_array)
        }
        "2" => {
            print!("Enter the path to the CSV file: ");
            let file_path = get_input()?;
            import_csv(&file_path)?
        }
        _ => return Err("Invalid choice".into()),
    };

    let mut calculator = SubnetCalculator::new(num_hosts_array);
    calculator.calculate(&ip, cidr)?;

    print_results(&calculator.subnets);

    if let Ok(true) = prompt_save() {
        save_results(&calculator.subnets)?;
    }

    Ok(())
}

fn get_network_input() -> Result<(String, u32), SubnetError> {
    print!("\nEnter the network address with CIDR notation (e.g. 192.168.1.0/24): ");
    let address = get_input()?;
    let ip_cidr: Vec<&str> = address.split('/').collect();
    if ip_cidr.len() != 2 {
        return Err(SubnetError::InvalidIpAddress(address));
    }
    let ip = ip_cidr[0].to_string();
    let cidr = ip_cidr[1]
        .parse()
        .map_err(|_| SubnetError::InvalidCidr(0))?;
    Ok((ip, cidr))
}

fn get_num_subnets() -> io::Result<u32> {
    print!("\nEnter the number of subnets: ");
    let num_subnets = get_input()?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    Ok(num_subnets)
}

fn get_num_hosts(num_subnets: u32) -> io::Result<Vec<u32>> {
    let mut num_hosts_array = Vec::new();
    for i in 0..num_subnets {
        print!("Enter the number of hosts for subnet #{}: ", i + 1);
        let num_hosts = get_input()?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        num_hosts_array.push(num_hosts);
    }
    Ok(num_hosts_array)
}

fn print_results(subnets: &[subnet::Subnet]) {
    for (i, field) in subnets.iter().enumerate() {
        println!("\n#{}: {}", i + 1, field.to_string());
        println!("{}", "-".repeat(50));
    }
}

fn prompt_save() -> io::Result<bool> {
    print!("\nDo you want to save the results? (y/n) (Supported formats: CSV (.csv), Markdown (.md)): ");
    let save = get_input()?.to_lowercase();
    Ok(save == "y" || save == "yes")
}

fn save_results(subnets: &[subnet::Subnet]) -> io::Result<()> {
    print!("Enter the file name (with the extension): ");
    let file_name = get_input()?;
    let save = SaveToFile::new(&file_name, subnets.to_vec());

    match file_name.split('.').last().unwrap().to_lowercase().as_str() {
        "md" => save.save_md()?,
        "csv" => save
            .save_csv()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file extension",
            ))
        }
    }

    println!("Results saved to {}", file_name);
    Ok(())
}

fn get_input() -> io::Result<String> {
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
