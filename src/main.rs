use std::error::Error;
use clap::Parser;
use std::path::Path;

enum FileData {
    CSV,
    Terminal,
    Nothing,
}

impl FileData {
    fn from_string(val: &str) -> FileData {
        match val.split('.').last().unwrap() {
            "csv" => FileData::CSV,
            "terminal" | "console" | "tty" => FileData::Terminal,
            _ => FileData::Nothing,
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The source file of the samples
    source: String,

    /// The input side of the resampling ratio
    input_ratio: String,

    /// The output side of the resampling ratio
    output_ratio: String,

    #[arg(short, long, default_value ="terminal")]
    /// The destination file of the samples
    dest: String,
    
    #[arg(short, long, default_value ="lerp")]
    interpolation_type: String,

    #[arg(short, long, default_value ="f32")]
    precision: String,
}

struct Resampler {
    input_samples: Vec<f32>,
    output_samples: Vec<f32>,
    ratio: f32,
    input_datatype: FileData,
    output_datatype: FileData,
    source_name: String,
    dest_name: String,
}

impl Resampler {
    fn from_args(args: &Args) -> Resampler {
        return Resampler {
            input_samples: vec![],
            output_samples: vec![],
            ratio: 
                args.input_ratio.parse::<f32>().unwrap() /
                args.output_ratio.parse::<f32>().unwrap(),
            input_datatype: FileData::from_string(&args.source),
            output_datatype: FileData::from_string(&args.dest),
            source_name: args.source.clone(),
            dest_name: args.dest.clone(),
        };
    }

    // Extends the end of input_samples by repeating the last
    // value n times
    fn repeat_last(&mut self, n: usize) {
        // Sets end values for algorithms that use later values
        if let Some(last_ref) = self.input_samples.last() {
            let last_val = *last_ref;
            self.input_samples.append(&mut vec![last_val; n]);
        }
    }

    // Removes the last n input_samples
    fn remove_last(&mut self, n: usize) {
        for _k in 0..n {
            self.input_samples.pop();
        }
    }

    // Performs piecewise constant interpolation and outputs
    // to output_samples
    fn do_piecewise_constant(&mut self) {
        let mut f: f32 = 0.0;
        
        while f <= (self.input_samples.len() - 1) as f32 {
            self.output_samples.push(piecewise_constant(
                    self.input_samples[f.floor() as usize],
                    self.input_samples[f.floor() as usize + 1],
                    f.fract()
                )
            );
            f += self.ratio;
        }
    }

    // Performs lerp and outputs to output_samples
    fn do_lerp(&mut self) {
        let mut f: f32 = 0.0;
        
        while f <= (self.input_samples.len() - 1) as f32 {
            let k = f.floor() as usize;

            self.output_samples.push(lerp(
                    self.input_samples[k],
                    self.input_samples[k + 1],
                    f.fract()
                )
            );
            f += self.ratio;
        }
    }

    // Performs cubic spline interpolation without the tangent
    // components and outputs to output_samples
    fn do_tangentless_hermite(&mut self) {
        let mut f: f32 = 0.0;
        
        while f <= (self.input_samples.len() - 1) as f32 {
            self.output_samples.push(tangentless_hermite(
                    self.input_samples[f.floor() as usize],
                    self.input_samples[f.floor() as usize + 1],
                    f.fract()
                )
            );
            f += self.ratio;
        }
    }

    // Performs Catmull-Rom cubic spline interpolation and
    // outputs to output_samples
    fn do_catmull_rom_spline(&mut self) {
        self.repeat_last(2);

        let mut f = 0.0;
        let m_get =
            | pos_behind: f32, pos_ahead: f32 | -> f32 {
                (pos_ahead - pos_behind) * 0.5
        };
        let mut m_current = 0.0;

        // TODO: properly handle beginning conditions of input_samples
        
        while f <= (self.input_samples.len() - 3) as f32 {
            self.output_samples.push((|| {
                let k = f.floor() as usize;
                let position = [
                    self.input_samples[k],
                    self.input_samples[k + 1],
                    self.input_samples[k + 2],
                ];

                let m_prev = m_current;
                m_current = m_get(position[0], position[2]);

                return even_hermite(
                    position[0],
                    position[1],
                    m_prev,
                    m_current,
                    f.fract(),
                );
            })());
            f += self.ratio;
        }

        self.remove_last(2);
    }

    // Gets data depending on the currently set data value
    fn data_get(&mut self) -> Result<(), Box<dyn Error>> {
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

    fn data_put(&mut self) -> Result<(), Box<dyn Error>> {
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

#[inline(always)]
fn piecewise_constant(start: f32, end: f32, t: f32) -> f32 {
    if t < 0.5 {
        start
    } else {
        end
    }
}

#[inline(always)]
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    (1.0 - t) * start + t * end
}

#[inline(always)]
fn tangentless_hermite(start: f32, end: f32, t: f32) -> f32 {
    let hermite = t * t * (3.0 - 2.0 * t);
    (1.0 - hermite) * start + hermite * end
}

#[inline(always)]
fn even_hermite(pos_0: f32, pos_1: f32, m_0: f32, m_1: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h00 = -h01 + 1.0;
    let h11 = t3 - t2;
    let h10 = t3 - 2.0 * t2 + t;
    return h00 * pos_0 + h10 * m_0 + h01 * pos_1 + h11 * m_1;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut resampler = Resampler::from_args(&args);

    resampler.data_get()?;

    match args.interpolation_type.to_lowercase().as_str() {
        "lerp" | "linear" | "l" => resampler.do_lerp(),
        "piecewise_constant" | "piecewise" | "piece" | "p"
            => resampler.do_piecewise_constant(),
        "tangentless_hermite" | "th" => resampler.do_tangentless_hermite(),
        "catmull_rom" | "cat" | "cr" => resampler.do_catmull_rom_spline(),
        _ => panic!("Interpolation type not found!"),
    }

    resampler.data_put()?;

    Ok(())
}
