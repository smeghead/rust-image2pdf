extern crate printpdf;

use getopts::Options;
use printpdf::*;
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

    for image_path in config.image_paths.iter() {
        println!("image_path: {}", image_path);
        let param = ImageParameter::new(image_path.to_string(), dpi).unwrap();

        let (page_index, layer_index) = doc.add_page(Mm(param.page_width), Mm(param.page_height), "Page 2, Layer 1");
        let current_layer = doc.get_page(page_index).get_layer(layer_index);
        let position = Mm::from(param.image.image.width.into_pt(dpi));
        param.image.add_to_layer(
            current_layer.clone(),
            None,
            Some(position),
            None,
            Some(param.scale),
            Some(param.scale),
            Some(dpi));
    }

    doc.save(&mut BufWriter::new(File::create(config.output_filename).unwrap())).unwrap();

    return Ok(());
}

#[derive(Debug)]
pub struct ImageParameter {
    pub image: Image,
    pub scale: f64,
    pub page_width: f64,
    pub page_height: f64,
    pub dpi: f64,
}
impl ImageParameter {
    pub fn new(filename: String, dpi: f64) -> Result<ImageParameter, &'static str> {
        let mut image_file: File = File::open(filename.clone()).unwrap();
        let decoder = JpegDecoder::new(&mut image_file).unwrap();
        let image = Image::try_from(decoder).unwrap();
        let width = Mm::from(image.image.width.into_pt(dpi)).0;
        let height = Mm::from(image.image.height.into_pt(dpi)).0;
        let scale_calculator = ScaleCalculator {width, height};
        let orientation = scale_calculator.get_orientation();
        let scale = scale_calculator.get_scale(&orientation);

        let page_width = match orientation {
            Orientation::Landscape {width, height: _} => width,
            Orientation::Portrait {width, height: _} => width,
        };
        let page_height = match orientation {
            Orientation::Landscape {width: _, height} => height,
            Orientation::Portrait {width: _, height} => height,
        };
        Ok(ImageParameter {image, scale, page_width, page_height, dpi})
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Orientation {
    Landscape {width: f64, height: f64},
    Portrait {width: f64, height: f64},
}

#[derive(Debug)]
struct ScaleCalculator {
    width: f64,
    height: f64,
}
impl ScaleCalculator {
    fn get_orientation(&self) -> Orientation {
        let diff = self.width - self.height;
        if diff > 0.0 {
            return Orientation::Landscape {width: 297.0, height: 210.0};
        }
        return Orientation::Portrait {width: 210.0, height: 297.0};
    }

    fn get_scale(&self, orientation: &Orientation) -> f64 {
        let w = match orientation {
            Orientation::Landscape {width, height: _} => width / self.width,
            Orientation::Portrait {width, height: _} => width / self.width,
        };
        let h = match orientation {
            Orientation::Landscape {width: _, height} => height / self.height,
            Orientation::Portrait {width: _, height} => height / self.height,
        };
        if  w < h {
            return w;
        }
        return h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_calculator_init() {
        let width = 1.0;
        let height = 2.0;
        let c: ScaleCalculator = ScaleCalculator {width, height};
        assert_eq!(c.width, 1.0);
        assert_eq!(c.height, 2.0);
    }

    #[test]
    fn scale_calculator_orientation_landscape() {
        let width = 100.0;
        let height = 50.0;
        let c: ScaleCalculator = ScaleCalculator {width, height};
        assert_eq!(c.get_orientation(), Orientation::Landscape {width: 297.0, height: 210.0});
    }

    #[test]
    fn scale_calculator_orientation_portrait() {
        let width = 100.0;
        let height = 150.0;
        let c: ScaleCalculator = ScaleCalculator {width, height};
        assert_eq!(c.get_orientation(), Orientation::Portrait {width: 210.0, height: 297.0});
    }

    #[test]
    fn scale_calculator_orientation_landscape_scale() {
        let width = 100.0;
        let height = 100.0;
        let c: ScaleCalculator = ScaleCalculator {width, height};
        let o = Orientation::Landscape {width: 297.0, height: 210.0};
        let expected = 210.0 / 100.0;
        assert_eq!(c.get_scale(&o), expected);
    }
}
