use UCTP::io::{read_input::read_json, normalize_input::normalize_data};
use UCTP::solver::simulated_annealing::SimulatedAnnealing;
use UCTP::io::output::print_schedule;
use UCTP::domain::config::Config;

fn main() {
    let config = Config::load().expect("Failed to load config.toml");
    let raw_input = match read_json(&config.file_name) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error, failed to load input: {}", e);
            std::process::exit(1);
        }
    };

    let normalized_input = normalize_data(raw_input);

    let sa = SimulatedAnnealing::new(normalized_input.clone() , config.start_temp, config.cooling_rate, config.max_iterations);
    let schedule = sa.run();
    let best_penalty = schedule.calculate_penalty(&normalized_input);

    print_schedule(&schedule, &normalized_input);
    println!("The penalty is {}", best_penalty);
}
