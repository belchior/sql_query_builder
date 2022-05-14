pub struct Formatter<'a> {
  pub comma: &'a str,
  pub lb: &'a str,
  pub indent: &'a str,
  pub space: &'a str,
}

impl<'a> Formatter<'a> {
  pub fn one_line() -> Self {
    Self {
      comma: ", ",
      lb: "",
      indent: "",
      space: " ",
    }
  }

  pub fn multi_line() -> Self {
    Self {
      comma: ", ",
      lb: "\n",
      indent: "\t",
      space: " ",
    }
  }
}

  }
}
