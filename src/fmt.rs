pub struct Formatter<'a> {
  pub sep: &'a str,
  pub indent: &'a str,
  pub comma: &'a str,
}

impl<'a> Formatter<'a> {
  pub fn new(pretty: bool) -> Self {
    match pretty {
      true => Self {
        sep: "\n",
        indent: "\t",
        comma: ", ",
      },
      false => Self {
        sep: " ",
        indent: "",
        comma: ",",
      },
    }
  }
}
