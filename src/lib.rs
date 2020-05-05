pub mod helper {
    pub fn section(title: &str) {
        let len = title.len();
        let dashes = "-".repeat(len);
        println!("\n+-{}-+", dashes);
        println!("| {} |", title);
        println!("+-{}-+", dashes);
    }

    pub fn subsection(title: &str) {
        let len = title.len();
        let dashes = "-".repeat(len+1);
        println!("\n{}:", title);
        println!("{}\n", dashes);
    }
}
