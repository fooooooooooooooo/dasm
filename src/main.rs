use std::fs::read;

use colored::{ColoredString, Colorize};
use iced_x86::{Decoder, DecoderOptions, Formatter, FormatterOutput, FormatterTextKind, IntelFormatter};

struct Output {
  vec: Vec<(String, FormatterTextKind)>,
}

impl Output {
  pub fn new() -> Self {
    Self { vec: Vec::new() }
  }
}

impl FormatterOutput for Output {
  fn write(&mut self, text: &str, kind: FormatterTextKind) {
    // This allocates a string. If that's a problem, just call print!() here
    // instead of storing the result in a vector.
    self.vec.push((String::from(text), kind));
  }
}

pub fn colorize(bytes: &[u8], rip: u64, bitness: u32) {
  let mut decoder = Decoder::with_ip(bitness, bytes, rip, DecoderOptions::NONE);

  let mut formatter = IntelFormatter::new();
  formatter.options_mut().set_first_operand_char_index(8);

  let mut output = Output::new();

  for instruction in &mut decoder {
    output.vec.clear();
    // The formatter calls output.write() which will update vec with text/colors
    formatter.format(&instruction, &mut output);

    for (text, kind) in output.vec.iter() {
      print!("{}", get_color(text.as_str(), *kind));
    }

    println!();
  }
}

fn get_color(s: &str, kind: FormatterTextKind) -> ColoredString {
  match kind {
    FormatterTextKind::Directive | FormatterTextKind::Keyword => s.bright_yellow(),
    FormatterTextKind::Prefix | FormatterTextKind::Mnemonic => s.bright_red(),
    FormatterTextKind::Register => s.bright_blue(),
    FormatterTextKind::Number => s.bright_cyan(),
    FormatterTextKind::LabelAddress | FormatterTextKind::FunctionAddress => s.bright_green(),
    FormatterTextKind::Text => s.green(),
    FormatterTextKind::Operator => s.bright_magenta(),
    _ => s.white(),
  }
}

const BITNESS: u32 = 64;
const RIP: u64 = 0x0;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
  #[clap()]
  pub input: String,
  #[clap(short, long)]
  pub bitness: Option<u32>,
  #[clap(short, long)]
  pub rip: Option<u64>,
}

fn main() {
  let args = Args::parse();

  let bytes = read(args.input).unwrap();
  let rip = args.rip.unwrap_or(RIP);
  let bitness = args.bitness.unwrap_or(BITNESS);

  colorize(&bytes, rip, bitness);
}
