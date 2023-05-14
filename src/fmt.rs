pub struct Formatter<'a> {
  pub comma: &'a str,
  pub hr: &'a str, // horizontal rule
  pub indent: &'a str,
  pub lb: &'a str, // line break
  pub space: &'a str,
}

pub fn one_line<'a>() -> Formatter<'a> {
  Formatter {
    comma: ", ",
    hr: "",
    indent: "",
    lb: "",
    space: " ",
  }
}

pub fn multiline<'a>() -> Formatter<'a> {
  Formatter {
    comma: ", ",
    hr: "-- ------------------------------------------------------------------------------\x1b[0m",
    indent: "  ",
    lb: "\n",
    space: " ",
  }
}

pub fn colorize(query: String) -> String {
  let sql_syntax: [(fn(&str) -> String, &str, &str); 61] = [
    (blue, "AND ", "and "),
    (blue, "COMMIT", "commit"),
    (blue, "CROSS ", "cross "),
    (blue, "DELETE ", "delete "),
    (blue, "EXCEPT ", "except "),
    (blue, "FROM ", "from "),
    (blue, "FULL ", "full "),
    (blue, "GROUP ", "group "),
    (blue, "HAVING ", "having "),
    (blue, "INNER ", "inner "),
    (blue, "INSERT ", "insert "),
    (blue, "INTERSECT ", "intersect "),
    (blue, "INTO ", "into "),
    (blue, "ISOLATION ", "isolation "),
    (blue, "JOIN ", "join "),
    (blue, "LEFT ", "left "),
    (blue, "LIMIT ", "limit "),
    (blue, "OFFSET ", "offset "),
    (blue, "ORDER ", "order "),
    (blue, "OVERRIDING ", "overriding "),
    (blue, "READ ONLY", "read only"),
    (blue, "READ WRITE", "read write"),
    (blue, "RELEASE ", "release "),
    (blue, "RETURNING ", "returning "),
    (blue, "RIGHT ", "right "),
    (blue, "ROLLBACK", "rollback"),
    (blue, "SAVEPOINT", "savepoint"),
    (blue, "SELECT ", "select "),
    (blue, "SET ", "set "),
    (blue, "START ", "start "),
    (blue, "TRANSACTION", "transaction"),
    (blue, "UNION ", "union "),
    (blue, "UPDATE ", "update "),
    (blue, "VALUES ", "values "),
    (blue, "WHERE ", "where "),
    (blue, "WITH ", "with "),
    (blue, " ALL", " all"),
    (blue, " ASC", " asc"),
    (blue, " AS", " as"),
    (blue, " BY", " by"),
    (blue, " COMMITTED", " committed"),
    (blue, " CONFLICT", " conflict"),
    (blue, " DEFERRABLE", " deferrable"),
    (blue, " DESC", " desc"),
    (blue, " DO", " do"),
    (blue, " DISTINCT", " distinct"),
    (blue, " FIRST", " first"),
    (blue, " IN", " in"),
    (blue, " LAST", " last"),
    (blue, " LEVEL", " level"),
    (blue, " NOTHING", " nothing"),
    (blue, " ON ", " on "),
    (blue, " OR ", " or "),
    (blue, " OUTER", " OUTER"),
    (blue, " SERIALIZABLE ", " serializable"),
    (blue, " REPEATABLE", " repeatable"),
    (blue, " USING", " using"),
    (blue, " UNCOMMITTED", " uncommitted"),
    (comment_start, "--", "--"),
    (comment_start, "/*", "/*"),
    (comment_end, "*/", "*/"),
  ];

  let mut query = sql_syntax.iter().fold(query, |acc, item| {
    let (color_fn, text_upper, text_lower) = item;
    acc
      .replace(text_upper, &color_fn(text_upper))
      .replace(text_lower, &color_fn(text_lower))
  });

  for index in 1..=10 {
    let arg_number = format!("${index}");
    query = query.replace(&arg_number, &bold(&arg_number))
  }

  query
}

pub fn format(query: String, fmts: &Formatter) -> String {
  let template = format!("{0}{1}{0}{query}{0}{1}{0}", fmts.lb, fmts.hr);
  let template = colorize(template);
  template
}

fn blue(text: &str) -> String {
  format!("\x1b[34;1m{text}\x1b[0m")
}

fn bold(text: &str) -> String {
  format!("\x1b[0;1m{text}\x1b[0m")
}

fn comment_start(text: &str) -> String {
  format!("\x1b[32;2m{text}")
}

fn comment_end(text: &str) -> String {
  format!("\x1b[32;2m{text}\x1b[0m")
}
