use hound;
use rand::Rng;
use std::path::Path;
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short,long, value_parser, num_args = 1.., value_delimiter = ' ')]
    filenames: Vec<String>,

    #[arg(short,long, default_value_t = 16)]
    bits: u16,

    #[arg(short,long, default_value_t = String::from(""))]
    destination: String,


    #[arg(short,long,default_value_t = false)]
    verbose: bool,
}


fn main() {

    let input = Args::parse();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: input.bits,
        sample_format: hound::SampleFormat::Int,
    };
 
    let dest = input.destination.clone();
    let verbosity = input.verbose.clone();
   
    for filename in input.filenames {
        bitconverter(&filename, spec, verbosity, dest.clone());
    }
    println!("Finished processing!")
}

fn bitconverter(file: &str, spec: hound::WavSpec, verbose: bool, destination: String) {

    if !Path::new(file).exists() {
        panic!("Non-existing file: {}", file);
    }

    match spec.bits_per_sample {
        n if (n == 8) | (n == 16) | (n == 24) => n,
        _ => panic!("Please choose a valid bit-rate!"),
    };

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

    let mut prefix = "".to_string();
    if destination != "" {
        prefix = format!("{}{}", destination , "/".to_string());
    }


    let newfilename = format!("{}{}{}",prefix,file.strip_suffix(".wav").expect("File does not have .wav extension"),suffix);

    let mut writer = hound::WavWriter::create(newfilename.clone(),spec).unwrap();

    let amplitude = 0.9; 

    // Calculating bit conversion scale factor:
    let originalbitsize = reader.as_ref().unwrap().spec().bits_per_sample;

    let base: i32 = 2;
    let scale: f64 = (base.pow((originalbitsize - spec.bits_per_sample).into())).into();
    
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
            32 => writer.write_sample((scaled_dithered_sample * amplitude) as i32).unwrap(),
            _ => panic!("Bit-rate is invalid!"),
        }
    }
   
    match writer.finalize() {
        Ok(file) => file,
        Err(e) => println!("Finalization faild due to: {}", e),
    };


    if verbose {

        match spec.bits_per_sample {
            n if n <= 8 => println!("Bit-rate is {} and is saved as 8-bit", n),
            n if n <= 16 =>  println!("Bit-rate is {} and is saved as 16-bit", n),
            n if n <= 24 =>  println!("Bit-rate is {} and is saved as 24-bit", n),
            n =>  println!("Bit-rate is {} and is saved as 32-bit", n),
        }; 
    };

}


#[test]
fn test8bit(){
    let filename = "testfiles/ND2sample1.wav";

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 8,
        sample_format: hound::SampleFormat::Int,
    };

    bitconverter(&filename, spec, false , "".to_string());
    let readercheck = hound::WavReader::open("ND2sample1_8bit.wav");
        // println!("Info on processed file: ");
        // println!("sample rate: {}", readercheck.as_ref().unwrap().spec().sample_rate);
        println!("bits: {}", readercheck.as_ref().unwrap().spec().bits_per_sample);


    assert_eq!(spec.bits_per_sample,readercheck.as_ref().unwrap().spec().bits_per_sample);

}

#[test]
fn test16bit(){
    let filename = "testfiles/ND2sample1.wav";

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    bitconverter(&filename, spec, false , "".to_string());
    let readercheck = hound::WavReader::open("ND2sample1_16bit.wav");
        // println!("Info on processed file: ");
        // println!("sample rate: {}", readercheck.as_ref().unwrap().spec().sample_rate);
        println!("bits: {}", readercheck.as_ref().unwrap().spec().bits_per_sample);


    assert_eq!(spec.bits_per_sample,readercheck.as_ref().unwrap().spec().bits_per_sample);

}


#[test]
fn test24bit(){
    let filename = "testfiles/ND2sample1.wav";

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    bitconverter(&filename, spec, false, "".to_string() );
    let readercheck = hound::WavReader::open("ND2sample1_24bit.wav");
        // println!("Info on processed file: ");
        // println!("sample rate: {}", readercheck.as_ref().unwrap().spec().sample_rate);
        println!("bits: {}", readercheck.as_ref().unwrap().spec().bits_per_sample);


    assert_eq!(spec.bits_per_sample,readercheck.as_ref().unwrap().spec().bits_per_sample);

}


#[test]
#[should_panic]
fn testinvalidbit(){
    let filename = "testfiles/ND2sample1.wav";

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 12,
        sample_format: hound::SampleFormat::Int,
    };

    bitconverter(&filename, spec, false , "".to_string());

}
