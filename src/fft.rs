use std::f64::consts::PI;
use num::complex::Complex;
use num::pow;
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "./testplot.png";

const PI32: f32= PI as f32;


fn fast_fourier_transform(samples: Vec<Complex<f32>>, sample_rate: i32) -> Vec<Complex<f32>> {
    
    let n_full = samples.len();
   
    // calculating blocklength to make sure it is a power of 2.
    let n = pow(2,(n_full as f32).log2().floor() as usize);

    // selecting subset of samples with correct blocklength
    let sel_samples = &samples[0..n];

    if n == 1 {
        return sel_samples.to_vec();
    }

    // Windowing the samples using a hann function
    let windowing = (0..n).map(|i| 0.5 * (1.0 - (i as f32 * 2.0*PI32 / (n-1) as f32 ).cos()));
    let _w_samples: Vec<f32> = sel_samples
        .into_iter()
        .zip(windowing)
        .map(|(x,y)| x.re*y)
        .collect();

    // Converting to complex numbers
    let complex_samples: Vec<Complex<f32>> = sel_samples
        .into_iter()
        .map(|x| Complex::new(x.re,0.0))
        .collect();

    // calculating nth root of unity
    let x = Complex::new(0.0, -2.0 * PI32 / (n as f32));
    let omega = x.exp();

        
    let mut even = vec![];
    let mut odd = vec![];
     
    for j in 0..(n/2) {
        even.push(complex_samples[2*j]);
        odd.push(complex_samples[2*j+1]);
    }    
    
    even = fast_fourier_transform(even, sample_rate);
    odd = fast_fourier_transform(odd, sample_rate);

    let mut dft = vec![];

    let mut dft1 = vec![];
    let mut dft2 = vec![];

    for j in 0..(n/2) {
        dft1.push(even[j] + pow(omega,j) * odd[j]);
        dft2.push(even[j] - pow(omega,j) * odd[j]);
    }

    dft.append(&mut dft1);
    dft.append(&mut dft2);
    
    dft

}


fn plot_freq_spectrum(filename: String, samples: &mut Vec<i32>, sample_rate: i32 ) {
    
    let complex_samples: Vec<Complex<f32>> = samples
        .into_iter()
        .map(|x| Complex::new(*x as f32,0.0))
        .collect();
    
    let n = pow(2,(samples.len() as f32).log2().floor() as usize) as f32;

    let freq_resolution = n as f32 / sample_rate as f32  ; 
    let bandwidth: usize = sample_rate as usize / 2 as usize;

    let dft = &fast_fourier_transform(complex_samples, sample_rate as i32)[0..(n as usize / 2)];

    let (max_index,max_amplitude) = dft
        .into_iter()
        .map(|x| x.norm())
        .enumerate()
        .max_by(|(_,a),(_,b)| a.total_cmp(&b))
        .expect("No valid max frequency");  
 

    let plotvals = dft
        .into_iter()
        .enumerate()
        .map(|(i,v)| (i as f32 * freq_resolution as f32, v.norm()/max_amplitude));
    
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root_area.fill(&WHITE).expect("Can't fill");

    let root_area = root_area.titled("FFT spectrum", ("sans-serif", 60)).expect("Can't create area!");

    let (upper, _lower) = root_area.split_vertically(512);
    let mut cc = ChartBuilder::on(&upper)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("Frequency Content", ("sans-serif", 40))
        .build_cartesian_2d(0.0 .. bandwidth as f32, 0.0..1.0 as f32).expect("Can't build canvas");

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw().expect("Can't draw");

    cc.draw_series(LineSeries::new(plotvals, &RED)).expect("Can't draw")
        .label("Frequencies")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    

}


fn get_n_freqs(sample)

// Can be made into a property based test, with different freqs. 
#[test]
fn test_simple_sine() {
    
    let sample_rate = pow(2,12);  
    let time_points = 1;           

    let freq: f32 = 600.0;

    let bandwidth: usize = sample_rate / 2 as usize;

    let mut samples: Vec<Complex<f32>> = vec![];

    for t in (0 .. time_points * sample_rate).map(|x| x as f32 / sample_rate as f32) {

        let sample = (t * freq * 2.0 * PI as f32 ).sin();
        let amplitude = i32::MAX as f32;
        samples.push(Complex::new(sample * amplitude, 0.0));
    }

    // Sanitizing sample input length.
    let n = pow(2,(samples.len() as f32).log2().floor() as usize) as f32;

    let freq_resolution = n as f32 / sample_rate as f32  ; 

    let dft = &fast_fourier_transform(samples.clone(), sample_rate as i32)[0..(n as usize / 2)];


    let (max_index,max_amplitude) = dft
        .into_iter()
        .map(|x| x.norm())
        .enumerate()
        .max_by(|(_,a),(_,b)| a.total_cmp(&b))
        .expect("No valid max frequency");  
  

    println!("{:?}", max_index as f32 * freq_resolution );
    assert_eq!(max_index as f32 * freq_resolution,freq);
}




// Make a property based test on FFT and inverse. 
