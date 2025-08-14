pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb_lines: Vec<String> = vec![];
    for n in 0..list.len() {
        if n + 1 < list.len() {
            proverb_lines.push(format!(
                "For want of a {} the {} was lost.",
                list[n],
                list[n + 1]
            ));
        }
    }
    if list.len() > 0 {
        proverb_lines.push(format!("And all for the want of a {}.", list[0]));
    }
    proverb_lines.join("\n")
}
