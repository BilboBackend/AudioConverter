use hound;
use rand::Rng;
use std::path::Path;
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    filename: String,
}


fn main() {

    let file1 = Args::parse();
    convert24to16bit(&file1.filename);
}


fn convert24to16bit(file: &str) {

    if !Path::new(file).exists() {
        panic!("Non-existing file: {}", file);
    }
    let reader = hound::WavReader::open(file);
    
    

    println!("{}", reader.as_ref().unwrap().spec().bits_per_sample);
    println!("{}", reader.as_ref().unwrap().spec().sample_rate);
    
    let mpcspec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

     
    match reader {
        Ok(ref v) => v,
        Err(e) => {
            panic!("Error loading file! {}", e);
            },
    };


    let newfilename = format!("{}{}",file.strip_suffix(".wav").expect("File does not have .wav extension"),"_16bit.wav");
    println!("{}",newfilename);
    let mut writer = hound::WavWriter::create(newfilename,mpcspec).unwrap();

    let amplitude = 0.99; 

    // initializing stuff for dithering.
    let mut rng = rand::thread_rng();
    let scale: f64 = 256.0; // = 2^24 / 2^16. 
    let scalingfactor: f64 = 0.8;
    let mut err: f64 = 0.0;

    // dithering and writing samples.
    for sample in reader.expect("Can't read reader").samples::<i32>() {
        
        let dither: f64 = rng.gen_range(-1.0..1.0);
        let scaled_sample = sample.unwrap() /scale as i32;
        let scaled_dithered_sample = (scaled_sample as f64 + dither + scalingfactor * err).floor();

        err = (scaled_sample as f64 - scaled_dithered_sample).into();
        
        writer.write_sample((scaled_dithered_sample * amplitude) as i16).unwrap();
    }
    

    match writer.finalize() {
        Ok(file) => file,
        Err(e) => println!("Finalization faild due to: {}", e),
    } ;

    println!("Finished Processing!");
 
}
