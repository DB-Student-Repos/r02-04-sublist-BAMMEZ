#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.is_empty() ||
        second_list.windows(first_list.len()).any(|x| x == first_list) {
        Comparison::Sublist
    } else if second_list.is_empty() ||
        first_list.windows(second_list.len()).any(|x| x == second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}


