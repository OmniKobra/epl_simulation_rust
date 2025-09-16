mod matches;
mod table;
mod teams;

use matches::Match;
use teams::Team;

pub use table::*;

#[cfg(test)]
mod tests {
    use crate::Table;

    #[test]
    pub fn test_print_table() {
        let t = Table::new();
        println!("{t}")
    }
}
