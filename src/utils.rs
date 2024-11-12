use std::cmp::PartialEq;

pub(crate) fn push_unique<T: PartialEq>(list: &mut Vec<T>, value: T) {
  let prev_item = list.iter().find(|&item| *item == value);
  if prev_item.is_none() {
    list.push(value);
  }
}
