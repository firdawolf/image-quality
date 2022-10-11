use std::time::Instant;

use clap::Parser;
use image::io::Reader as ImageReader;
use ndarray::{ArrayBase, Dim, OwnedRepr};
use nshare::{self, ToNdarray3};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Original Image without deformation
    #[arg(short, long)]
    source: String,
    /// The compare image that is deformed
    #[arg(short, long)]
    compare: String,
    /// image quality assessment metrics ( psnr , ms-ssim , mse , qnr)
    #[arg(short, long)]
    metric: String,
}

fn main() {
    let now = Instant::now();
    let args = Args::parse();
    let mut format_source1 = None;
    let mut format_compare1 = None;
    std::thread::scope(|s| {
        s.spawn(|| {
            format_source1 = Some(
                ImageReader::open(args.source)
                    .expect("cannot convert image source")
                    .decode()
                    .unwrap()
                    .into_rgb8()
                    .into_ndarray3()
                    .mapv(|x| f32::from(x)),
            );
        });
        s.spawn(|| {
            format_compare1 = Some(
                ImageReader::open(args.compare)
                    .expect("cannot convert image compare")
                    .decode()
                    .unwrap()
                    .into_rgb8()
                    .into_ndarray3()
                    .mapv(|x| f32::from(x)),
            );
        });
    });
    let format_source = &format_source1.unwrap();
    let format_compare = &format_compare1.unwrap();
    // if format_source.dim() != format_compare.dim() {
    //     panic!("source image dimension is not equal to compare image dimension");
    // }
    // if format_source.len() != format_compare.len() {
    //     panic!("source image len is not equal to compare image len");
    // }
    match args.metric.as_str() {
        "mse" => {
            println!("value :{}", mse(format_source, format_compare))
        }
        "psnr" => {
            println!("value :{}", psnr(format_source, format_compare))
        }
        _ => {
            panic!("unsupported matric")
        }
    }
    println!("Total time took :{:?}ms", now.elapsed().as_millis());
}

fn mse(
    format_source: &ArrayBase<OwnedRepr<f32>, Dim<[usize; 3]>>,
    format_compare: &ArrayBase<OwnedRepr<f32>, Dim<[usize; 3]>>,
) -> f32 {
    let a = format_source - format_compare;
    let a = a.mapv(|a| a.powf(2.0));
    println!("Source len {:?}", format_source.len());
    a.mean().expect("cannot use mean op in mse")
}
fn psnr(
    format_source: &ArrayBase<OwnedRepr<f32>, Dim<[usize; 3]>>,
    format_compare: &ArrayBase<OwnedRepr<f32>, Dim<[usize; 3]>>,
) -> f32 {
    let mse_value = mse(format_source, format_compare);
    if mse_value == 0.0 {
        return 0.0;
    } else {
        let mut max = u8::MAX as f32;
        max = max.powf(2.0);
        let a = max / mse_value;
        return 10.0 * a.log10();
    }
}
