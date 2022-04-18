#[derive(Default, Clone)]
pub struct SelectBuilder<'a> {
  pub _from: Vec<String>,
  pub _join: Vec<String>,
  pub _limit: &'a str,
  pub _order_by: Vec<String>,
  pub _query: String,
  pub _raw: Vec<String>,
  pub _select: Vec<String>,
  pub _union: Vec<Self>,
  pub _where: Vec<String>,
  pub _with: Vec<(&'a str, Self)>,
}
