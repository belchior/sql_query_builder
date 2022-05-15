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

pub fn colorize(sql: String) -> String {
  let keywords: [(fn(&str) -> String, &str, &str); 34] = [
    (blue, "EXCEPT ", "except "),
    (blue, "CROSS ", "cros "),
    (blue, "FROM ", "from "),
    (blue, "FULL ", "full "),
    (blue, "GROUP ", "group "),
    (blue, "HAVING ", "having "),
    (blue, "INNER ", "inner "),
    (blue, "INTERSECT ", "intersect "),
    (blue, "JOIN ", "join "),
    (blue, "LEFT ", "left "),
    (blue, "LIMIT ", "limit "),
    (blue, "OFFSET ", "offset "),
    (blue, "ORDER ", "order "),
    (blue, "RIGHT ", "right "),
    (blue, "SELECT ", "select "),
    (blue, "UNION ", "union "),
    (blue, "WHERE ", "where "),
    (blue, "WITH ", "with "),
    (blue, " ALL", " all"),
    (blue, " AND", " and"),
    (blue, " AS", " as"),
    (blue, " ASC", " asc"),
    (blue, " BY", " by"),
    (blue, " DESC", " desc"),
    (blue, " DISTINCT", " distinct"),
    (blue, " FIRST", " first"),
    (blue, " IN", " in"),
    (blue, " LAST", " last"),
    (blue, " ON", " on"),
    (blue, " OUTER", " OUTER"),
    (blue, " USING", " using"),
    (comment_start, "--", "--"),
    (comment_start, "/*", "/*"),
    (comment_end, "*/", "*/"),
  ];

  let mut sql = keywords.iter().fold(sql, |acc, item| {
    let (color_fn, text_upper, text_lower) = item;
    acc
      .replace(text_upper, &color_fn(text_upper))
      .replace(text_lower, &color_fn(text_lower))
  });

  for index in 1..=10 {
    let arg_number = format!("${index}");
    sql = sql.replace(&arg_number, &bold(&arg_number))
  }

  sql
}

fn blue(text: &str) -> String {
  format!("\x1b[34;1m{text}\x1b[0m")
}

fn bold(text: &str) -> String {
  format!("\x1b[0;1m{text}\x1b[0m")
}

fn comment_start(text: &str) -> String {
  format!("\x1b[32;m{text}")
}

fn comment_end(text: &str) -> String {
  format!("\x1b[32;m{text}\x1b[0m")
}