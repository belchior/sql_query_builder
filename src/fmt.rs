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
  let sql_syntax: Vec<(fn(&str) -> String, &str, &str)> = vec![
    (blue, "COMMIT", "commit"),
    (blue, "COMMITTED", "committed"),
    (blue, "CONFLICT", "conflict"),
    (blue, "CONSTRAINT", "constraint"),
    (blue, "CREATE ", "create "),
    (blue, "DEFAULT", "default"),
    (blue, "DEFERRABLE", "deferrable"),
    (blue, "DELETE ", "delete "),
    (blue, "DISTINCT", "distinct"),
    (blue, "EXCEPT", "except"),
    (blue, "EXISTS", "exists"),
    (blue, "FROM ", "from "),
    (blue, "GROUP BY", "group by"),
    (blue, "HAVING", "having"),
    (blue, "INNER", "inner"),
    (blue, "INSERT", "insert"),
    (blue, "INTERSECT", "intersect"),
    (blue, "ISOLATION", "isolation"),
    (blue, "JOIN ", "join "),
    (blue, "LEFT ", "left "),
    (blue, "LIMIT ", "limit "),
    (blue, "NOTHING", "nothing"),
    (blue, "OFFSET", "offset"),
    (blue, "ORDER BY", "order by"),
    (blue, "OVERRIDING", "overriding"),
    (blue, "PRIMARY", "primary"),
    (blue, "READ ONLY", "read only"),
    (blue, "READ WRITE", "read write"),
    (blue, "RELEASE", "release"),
    (blue, "REPEATABLE", "repeatable"),
    (blue, "REPLACE", "REPLACE"),
    (blue, "RETURNING", "returning"),
    (blue, "RIGHT", "right"),
    (blue, "ROLLBACK", "rollback"),
    (blue, "SAVEPOINT", "savepoint"),
    (blue, "SELECT ", "select "),
    (blue, "SERIALIZABLE", "serializable"),
    (blue, "SET ", "set "),
    (blue, "START ", "start "),
    (blue, "TABLE", "table"),
    (blue, "TRANSACTION", "transaction"),
    (blue, "UNCOMMITTED", "uncommitted"),
    (blue, "UNION ", "union "),
    (blue, "UPDATE ", "update "),
    (blue, "VALUES ", "values "),
    (blue, "WHERE ", "where "),
    (blue, "WITH ", "with "),
    (blue, " ALL", " all"),
    (blue, " AND", " and"),
    (blue, " AS ", " as "),
    (blue, " ASC", " asc"),
    (blue, " CROSS", " cross"),
    (blue, " DESC", " desc"),
    (blue, " DO", " do"),
    (blue, " END", " end"),
    (blue, " FIRST", " first"),
    (blue, " FULL", " full"),
    (blue, " IF", " if"),
    (blue, " IN ", " in "),
    (blue, " INTO", " into"),
    (blue, " KEY", " key"),
    (blue, " LAST", " last"),
    (blue, " LEVEL", " level"),
    (blue, " NOT", " not"),
    (blue, " ON ", " on "),
    (blue, " OR ", " or "),
    (blue, " OUTER", " OUTER"),
    (blue, " UNIQUE", " unique"),
    (blue, " USING", " using"),
    (blue, " VARCHAR", " varchar"),
    (red, " NULL", " null"),
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

fn red(text: &str) -> String {
  format!("\x1b[91;2m{text}\x1b[0m")
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
