use UCTP::io::{read_input::read_json, normalize_input::normalize_data};
use UCTP::solver::simulated_annealing::SimulatedAnnealing;
use UCTP::io::output::print_schedule;

fn main() {
    let raw_input = read_json();

    let normalized_input = normalize_data(raw_input);

    let sa = SimulatedAnnealing::new(normalized_input.clone() , 2000.0, 0.9995, 10000000);
    let schedule = sa.run();
    let best_penalty = schedule.calculate_penalty(&normalized_input);

    println!("The penalty is {}", best_penalty);
    print_schedule(&schedule, &normalized_input);
}
