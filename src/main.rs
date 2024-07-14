use std::error::Error;
use clap::Parser;
use resampler::resamplers::core;

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


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut resampler = core::Resampler::from_args(
        args.source,
        args.dest,
        args.input_ratio.parse::<f32>().unwrap(),
        args.output_ratio.parse::<f32>().unwrap(),
    );
    
    resampler.data_get()?;

    resampler.do_resample(args.interpolation_type.clone());

    resampler.data_put()?;

    Ok(())
}
