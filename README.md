# rust-pdfpackman #

This is a cli command to convert multiple image files to a PDF file.

## Build ##

```
$ git clone https://github.com/smeghead/rust-pdfpackman.git
$ cd rust-pdfpackman
$ cargo build --release
$ ./target/release/pdfpackman --help
```

## Usage ##

```
Usage: pdfpackman [options] FILE [FILE..]

Options:
    -o, --output NAME   set output file name
    -h, --help          print this help menu
```

### example ###

```
pdfpackman -o output.pdf sample.jpg sample.png
```

## known issues ##

- a png file has a alpha channel is not apear in the output pdf file.

