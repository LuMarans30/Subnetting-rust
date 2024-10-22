use std::fs::File;
use std::io::{self, Write};

use crate::subnet::Subnet;

/// Struct that contains the file path of the md or csv file and the subnets array
pub struct SaveToFile {
    filepath: String,
    subnets: Vec<Subnet>,
}

/// Saves the subnets to a file
impl SaveToFile {
    pub fn new(filepath: &str, subnets: Vec<Subnet>) -> SaveToFile {
        SaveToFile {
            filepath: filepath.to_string(),
            subnets,
        }
    }

    /// Saves the subnets information to a file in Markdown format (table)
    pub fn save_md(&self) -> io::Result<()> {
        let mut file = File::create(&self.filepath)?;
        for (i, field) in self.subnets.iter().enumerate() {
            writeln!(
                file,
                "## Subnet {}:\n\n{}\n{}",
                i,
                field.to_markdown_table(),
                "-".repeat(3)
            )?;
        }
        Ok(())
    }

    /// Saves the subnets information to a file in CSV format using the serde serializer
    pub fn save_csv(&self) -> Result<(), csv::Error> {
        let mut wtr = csv::Writer::from_path(&self.filepath)?;
        for field in &self.subnets {
            wtr.serialize(field)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
