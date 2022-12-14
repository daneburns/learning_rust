use regex::Regex;
use colored::Colorize;
use clap::{App, Arg};

fn main() {
  let args = App::new("grep-lite")
    .version("0.1")
    .about("searches for patterns")
    .arg(Arg::with_name("pattern")
      .help("The pattern to search for")
      .takes_value(true)
      .required(true))
    .get_matches();

  let pattern = args.value_of("pattern").unwrap();
  let re = Regex::new(pattern).unwrap();
 
  let quote = "Every face, every shop, bedroom window, public-house, and 
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
 
  for (i, line) in quote.lines().enumerate() {
    let contains_substring = re.find(line);
    match contains_substring {
 
        Some(x) => println!("{}: {}{}{}", i + 1, &line[0..x.start()], &line[x.range()].bright_red().bold().underline(), &line[x.end()..line.len()]),
        None => (),
    }
  }
}
