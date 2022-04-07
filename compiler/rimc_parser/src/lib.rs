extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "rim.pest"]
pub struct RimParser;

#[cfg(test)]
mod tests {
  use pest::Parser;

  use crate::RimParser;
  use crate::Rule;

  #[test]
  fn parse_rim_string() {
    let successfull_string_parse = RimParser::parse(
      Rule::File,
      concat!(
        "const char_space = ' ';",
        "mut char_char = 'c';",
        "const char_emoji = '👀';",
        "",
        "mut string;",
        "mut string_empty = \"\";",
        "const hello_world = \"",
        "  Hello World!",
        "\";",
        "",
        "mut true = true;",
        "const false = false;",
        "",
        "const integer = 82364365;",
        "mut fd = 0.9347;",
        "const fd = .9347;",
        "mut dushf = 2375.2309;"
      ),
    );
    println!("{:#?}", successfull_string_parse);
    assert!(successfull_string_parse.is_ok());
  }
}
