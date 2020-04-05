mod format;
mod parse;
mod utils;

pub fn main() {
  let elements = parse::parse(
    "document {
  title {Sample title.}
  mlml-version {1}
} {
  section {Formatting lines} {
    section {Text} {
      In MLML, text is pretty easy to format: With the current indention, word
      after word is just added. If a word doesn't fit into the limit and it's
      not the only word on the line, a newline is inserted before and it's
      indented.
    }
    section {Blocks} {
      Blocks have three different formatting modes:
      inline-section {Inlining.} {
        If a block contains only text and no other blocks, it can be inlined,
        meaning that it may start in the middle of a line while still breaking
        its content:
        code {mlml} {
          This is text with some bold{blocks that are
          formatted inline}. Notice that italic{their
          contents may break}.
        }
      }
      inline-section {One-lining} {}
    }
  }
  image {
    url {https://someimage.com/image}
    alternative {Alternative text}
  }
  code {mlml} {...}
}
",
  );
  for element in &elements {
    println!("{:?}", element);
  }
  println!(
    "{}",
    format::format(
      &elements,
      &format::Context {
        limit: 80,
        indentation: "  ".to_string(),
      }
    )
  );
}
