use std::error::Error;
use std::path::Path;
use super::core;
use crate::filedata::FileData;

impl core::Resampler {
    // Gets data depending on the currently set data value
    pub fn data_get(&mut self) -> Result<(), Box<dyn Error>> {
        match self.input_datatype {
            FileData::CSV => self.csv_get(),
            _ => panic!("data_get() failed!"),
        };

        Ok(())
    }

    fn csv_get(&mut self) -> Result<(), Box<dyn Error>> {
        let mut csv_reader = csv::Reader::from_path(Path::new(self.source_name.as_str()))
            .expect("Could not get csv reader!");
        for result in csv_reader.records() {
            self.input_samples.push(result?.get(0).unwrap().parse::<f32>()?);
        }

        Ok(())
    }

    pub fn data_put(&mut self) -> Result<(), Box<dyn Error>> {
        match self.output_datatype {
            FileData::CSV => self.csv_put(),
            FileData::Terminal => Ok(println!("{:?}", self.output_samples)),
            _ => panic!("Undefined file data!"),
        }
    }

    fn csv_put(&mut self) -> Result<(), Box<dyn Error>> {
        let mut csv_writer = csv::Writer::from_path(Path::new(self.dest_name.as_str()))
            .expect("Could not get csv writer!");
        for val in self.output_samples.iter() {
            csv_writer.write_record(&[format!("{val}")]);
        }

        Ok(())
    }
}
