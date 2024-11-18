use std::cmp::PartialEq;

pub(crate) fn push_unique<T: PartialEq>(list: &mut Vec<T>, value: T) {
  let prev_item = list.iter().find(|&item| *item == value);
  if prev_item.is_none() {
    list.push(value);
  }
}

pub(crate) fn join(list: &Vec<String>, sep: &str) -> String {
  list
    .iter()
    .filter(|item| item.is_empty() == false)
    .map(|item| item.as_str())
    .collect::<Vec<_>>()
    .join(sep)
}
