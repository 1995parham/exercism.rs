pub fn build_proverb(list: &[&str]) -> String {
    let mut res = String::new();

    for (w, l) in list.iter().zip(list.iter().skip(1)) {
        res += format!("For want of a {} the {} was lost.\n", w, l).as_str();
    }
    if list.len() > 0 {
        res += format!("And all for the want of a {}.", list[0]).as_str();
    }

    res
}
