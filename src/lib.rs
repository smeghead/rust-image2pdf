extern crate printpdf;

use getopts::Options;
use printpdf::*;
//use std::io::Cursor;
use image::jpeg::JpegDecoder;
use std::fs::File;
use std::io::BufWriter;

pub struct Config {
    pub image_paths: Vec<String>,
    pub output_filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optopt("o", "output", "set output file name", "NAME");
        opts.optflag("h", "help", "print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(_) => { return Err("failed to parse options.") }
        };
        if matches.opt_present("h") {
            print_usage(&program, opts);
            return Err("help!");
        }
        let output_filename = match matches.opt_str("o") {
            Some(s) => s,
            None => String::from("output.pdf"),
        };
        let image_paths = matches.free;

        Ok(Config { image_paths, output_filename })
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    eprint!("{}", opts.usage(&brief));
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    let doc = PdfDocument::empty("printpdf graphics test");

    let dpi = 300.0;
    // currently, the only reliable file formats are bmp/jpeg/png
    // this is an issue of the image library, not a fault of printpdf

    for image_path in config.image_paths.iter() {
        let (page_index, layer_index) = doc.add_page(Mm(210.0), Mm(297.0),"Page 2, Layer 1");
        let current_layer = doc.get_page(page_index).get_layer(layer_index);
        println!("image_path: {}", image_path);
        let mut image_file = File::open(image_path).unwrap();
        let decoder = JpegDecoder::new(&mut image_file).unwrap();
        let image2 = Image::try_from(decoder).unwrap();

        println!("image2 width: {:?} height: {:?}", image2.image.width.into_pt(dpi), image2.image.height.into_pt(dpi));
        image2.add_to_layer(current_layer.clone(), None, Some(Mm(100.0)), None, Some(2.0), Some(2.0), Some(dpi));
    }

    doc.save(&mut BufWriter::new(File::create(config.output_filename).unwrap())).unwrap();

    return Ok(());
}
