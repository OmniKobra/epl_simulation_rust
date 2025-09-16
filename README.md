# EPL-Simulator ⚽️

This is a Rust-based CLI application that simulates an English Premier League (EPL) season. It generates a full season's worth of matches, simulates the results, and displays the final league table with updated standings.

## ✨ Features

* **Season Simulation**: Simulates a complete EPL season with 20 teams.
* **Realistic Standings**: Calculates and displays team standings, including points, wins, draws, losses, goals for, goals against, and goal difference. The table is sorted based on points, with goal difference as a tie-breaker.
* **CLI Interface**: A straightforward command-line interface to run the simulation and view the results.

## 🚀 How to Run

### Prerequisites

You need to have **Rust** installed on your system. If you don't have it, you can install it using `rustup`.

### Running the Simulator

1.  Clone the repository:
    ```bash
    git clone https://github.com/OmniKobra/epl_simulation_rust
    cd epl_simulator_rust
    ```
2.  Run the project using `cargo`:
    ```bash
    cargo run
    ```
    This command will compile and execute the `main.rs` file, which runs the simulation and prints the final league table to your console.

## 📂 Project Structure

* `src/main.rs`: The main entry point of the application. It creates a `Table` instance, runs the simulation, and prints the results.
* `src/lib.rs`: The library crate for the project. It defines the public modules.
* `src/table/mod.rs`: Contains the `Table` struct and its implementation, which manages the teams, matches, and the league standings.
* `src/teams/mod.rs`: This moduledefines the `Team` struct and its methods for managing team-specific data like points, goals, etc.
* `src/matches/mod.rs`: This module defines the `Match` and `Stats` struct and their methods for simulating match outcomes.

## 📝 How it Works

The core of the simulation is the **`Table`** struct. When **`Table::new()`** is called, it initializes 20 teams based on a predefined list of EPL teams and generates all the matches for a full season.

The **`sim_all_matches()`** method then iterates through each match, simulates a random outcome (win, loss, or draw), and updates the points for the respective teams. Finally, **`update_table()`** re-sorts the teams based on points and goal difference, giving the final league standings.

The **`show_teams_stats()`** method then formats this data into a clean, readable table that is displayed on the console.