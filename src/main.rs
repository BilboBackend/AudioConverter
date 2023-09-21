use hound;
use rand::Rng;
use std::path::Path;
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    filename: String,

    #[arg(short,long, default_value_t = 16)]
    bits: u16,
    
    #[arg(short,long,default_value_t = true)]
    verbose: bool,
}


fn main() {

    let file = Args::parse();
    
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: file.bits,
        sample_format: hound::SampleFormat::Int,
    };

    bitconverter(&file.filename, spec, file.verbose);
}

fn bitconverter(file: &str, spec: hound::WavSpec, verbose: bool) {

    if !Path::new(file).exists() {
        panic!("Non-existing file: {}", file);
    }


    let reader = hound::WavReader::open(file);
    
    match reader {
        Ok(ref v) => v,
        Err(e) => {
            panic!("Error loading file! {}", e);
            },
    };

    let suffix = match spec.bits_per_sample {
        8 => "_8bit.wav",
        16 => "_16bit.wav",
        24 => "_24bit.wav",
        _ => "no_change",
    };



    let newfilename = format!("{}{}",file.strip_suffix(".wav").expect("File does not have .wav extension"),suffix);

    let mut writer = hound::WavWriter::create(newfilename.clone(),spec).unwrap();

    let amplitude = 0.9; 

    // Calculating bit conversion scale factor:
    let originalbitsize = reader.as_ref().unwrap().spec().bits_per_sample;
    println!("{}", 2^originalbitsize);

    let base: i32 = 2;
    let scale: f64 = (base.pow((originalbitsize - spec.bits_per_sample).into())).into();
    
    println!("{}", scale);
    // initializing stuff for dithering.
    let mut rng = rand::thread_rng();
    let scalingfactor: f64 = 0.8;
    let mut err: f64 = 0.0;

    // dithering and writing samples.
    for sample in reader.expect("Can't read reader").samples::<i32>() {
        
        let dither: f64 = rng.gen_range(-1.0..1.0);
        let scaled_sample = sample.unwrap() /scale as i32;
        let scaled_dithered_sample = (scaled_sample as f64 + dither + scalingfactor * err).floor();

        err = (scaled_sample as f64 - scaled_dithered_sample).into();
       
        match spec.bits_per_sample {
            8 => writer.write_sample((scaled_dithered_sample * amplitude) as i8).unwrap(),
            16 =>  writer.write_sample((scaled_dithered_sample * amplitude) as i16).unwrap(),
            24 =>  writer.write_sample((scaled_dithered_sample * amplitude) as i32).unwrap(),
            _ => writer.write_sample((scaled_dithered_sample * amplitude) as i32).unwrap(),
        }
    }
    

    match writer.finalize() {
        Ok(file) => file,
        Err(e) => println!("Finalization faild due to: {}", e),
    } ;


    if verbose {
        let readercheck = hound::WavReader::open(newfilename.clone());
        println!("Info on processed file: ");
        println!("sample rate: {}", readercheck.as_ref().unwrap().spec().sample_rate);
        println!("bits: {}", readercheck.as_ref().unwrap().spec().bits_per_sample);
    }

 
    println!("Finished Processing!");
}
