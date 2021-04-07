extern crate printpdf;
extern crate getopts;

use getopts::Options;
use printpdf::*;
//use std::io::Cursor;
use image::jpeg::JpegDecoder;
use std::env;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    for image_path in config.image_paths.iter() {
        println!("image_path: {}", image_path);
    }

    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let dpi = 300.0;
    // currently, the only reliable file formats are bmp/jpeg/png
    // this is an issue of the image library, not a fault of printpdf

    let mut image_file = File::open("sample.jpg").unwrap();
    let decoder = JpegDecoder::new(&mut image_file).unwrap();
    let image2 = Image::try_from(decoder).unwrap();

    println!("image2 width: {:?} height: {:?}", image2.image.width.into_pt(dpi), image2.image.height.into_pt(dpi));
//    println!("image2  : {:?} ", doc);
    // layer,     
    image2.add_to_layer(current_layer.clone(), None, Some(Mm(100.0)), None, Some(2.0), Some(2.0), Some(dpi));

    doc.save(&mut BufWriter::new(File::create(config.output_filename).unwrap())).unwrap();
    println!("Hello, world!");
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

struct Config {
    image_paths: Vec<String>,
    output_filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optopt("o", "output", "set output file name", "NAME");
        opts.optflag("h", "help", "print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(_) => { panic!("failed to parse options.") }
        };
        if matches.opt_present("h") {
            print_usage(&program, opts);
            panic!("help!")
        }
        let output_filename = match matches.opt_str("o") {
            Some(s) => s,
            None => String::from("output.pdf"),
        };
        let image_paths = matches.free;

        Config { image_paths, output_filename }
    }
}
