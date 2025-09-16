use epl_simulator::Table;

fn main() {
    let mut t = Table::new();
    t.sim_all_matches();
    println!("{t}");
}
