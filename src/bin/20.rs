use num::Integer;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy)]
enum Signal {
    LowPulse,
    HighPulse,
}

#[derive(Debug)]
enum Module {
    Broadcaster(Vec<String>),
    FlipFlop {
        online: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, Signal>,
        outputs: Vec<String>,
    },
}

impl Module {
    fn signal(&mut self, source: &str, s: Signal) -> Vec<(String, Signal)> {
        match self {
            Self::Broadcaster(outputs) => outputs.iter().map(|name| (name.clone(), s)).collect(),
            Self::FlipFlop { online, outputs } if matches!(s, Signal::LowPulse) => {
                if *online {
                    *online = false;
                    outputs
                        .iter()
                        .map(|name| (name.clone(), Signal::LowPulse))
                        .collect()
                } else {
                    *online = true;
                    outputs
                        .iter()
                        .map(|name| (name.clone(), Signal::HighPulse))
                        .collect()
                }
            }
            Self::Conjunction { inputs, outputs } => {
                let cached = inputs.get_mut(source).unwrap();
                *cached = s;
                let output_signal = if inputs.values().all(|s| matches!(s, Signal::HighPulse)) {
                    Signal::LowPulse
                } else {
                    Signal::HighPulse
                };
                outputs
                    .iter()
                    .map(|name| (name.clone(), output_signal))
                    .collect()
            }
            _ => Vec::new(),
        }
    }
}

fn parse_module(value: &str) -> (&str, Module) {
    let (module, outputs) = value.split_once(" -> ").unwrap();
    let outputs = outputs.split(", ").map(|s| s.to_owned()).collect();

    match module {
        "broadcaster" => (module, Module::Broadcaster(outputs)),
        m if m.starts_with('%') => (
            &m[1..],
            Module::FlipFlop {
                online: false,
                outputs,
            },
        ),
        m if m.starts_with('&') => (
            &m[1..],
            Module::Conjunction {
                inputs: HashMap::new(),
                outputs,
            },
        ),
        _ => panic!("unknown module"),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut modules: HashMap<_, _> = input.lines().map(parse_module).collect();
    let mut conjunction_inputs_map = HashMap::new();
    for (name, module) in modules.iter() {
        let outputs = match module {
            Module::Broadcaster(output)
            | Module::FlipFlop {
                outputs: output, ..
            }
            | Module::Conjunction {
                outputs: output, ..
            } => output,
        };

        for n in outputs {
            if let Some(Module::Conjunction { .. }) = modules.get(n.as_str()) {
                let entry = conjunction_inputs_map
                    .entry(n.to_owned())
                    .or_insert_with(HashMap::new);
                entry.insert(name.to_string(), Signal::LowPulse);
            }
        }
    }
    for (name, conjunction_inputs) in conjunction_inputs_map {
        if let Some(Module::Conjunction { inputs, outputs: _ }) = modules.get_mut(name.as_str()) {
            *inputs = conjunction_inputs;
        } else {
            panic!("invalid code");
        }
    }

    let mut low_pulse_counts = 0;
    let mut high_pulse_counts = 0;
    let mut cache = HashMap::new();

    let mut button_pushed = 0;
    while button_pushed != 1000 {
        let mut signals_queue = VecDeque::new();
        signals_queue.push_back(vec![(
            "button".to_string(),
            "broadcaster".to_string(),
            Signal::LowPulse,
        )]);
        button_pushed += 1;

        while let Some(signals) = signals_queue.pop_front() {
            for (source, target, signal) in signals {
                match signal {
                    Signal::LowPulse => low_pulse_counts += 1,
                    Signal::HighPulse => high_pulse_counts += 1,
                }
                if let Some(module) = modules.get_mut(target.as_str()) {
                    let outputs = module.signal(source.as_str(), signal);
                    signals_queue.push_back(
                        outputs
                            .into_iter()
                            .map(|(new_target, signal)| (target.clone(), new_target, signal))
                            .collect(),
                    )
                }
            }
        }

        let mut some_flip_flop_online = false;
        for module in modules.values() {
            if let Module::FlipFlop { online, .. } = module {
                some_flip_flop_online = some_flip_flop_online || *online;
            }
        }

        if some_flip_flop_online {
            cache.insert(button_pushed, (low_pulse_counts, high_pulse_counts));
        } else {
            let rounds = 1000 / button_pushed;
            low_pulse_counts *= rounds;
            high_pulse_counts *= rounds;

            let remainder = 1000 % button_pushed;
            if remainder > 0 {
                low_pulse_counts += cache.get(&remainder).unwrap().0;
                high_pulse_counts += cache.get(&remainder).unwrap().1;
            }

            break;
        }
    }

    Some(low_pulse_counts * high_pulse_counts)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut modules: HashMap<_, _> = input.lines().map(parse_module).collect();
    let mut conjunction_inputs_map = HashMap::new();
    for (name, module) in modules.iter() {
        let outputs = match module {
            Module::Broadcaster(output)
            | Module::FlipFlop {
                outputs: output, ..
            }
            | Module::Conjunction {
                outputs: output, ..
            } => output,
        };

        for n in outputs {
            if let Some(Module::Conjunction { .. }) = modules.get(n.as_str()) {
                let entry = conjunction_inputs_map
                    .entry(n.to_owned())
                    .or_insert_with(HashMap::new);
                entry.insert(name.to_string(), Signal::LowPulse);
            }
        }
    }
    for (name, conjunction_inputs) in conjunction_inputs_map {
        if let Some(Module::Conjunction { inputs, outputs: _ }) = modules.get_mut(name.as_str()) {
            *inputs = conjunction_inputs;
        } else {
            panic!("invalid code");
        }
    }

    let mut result = 1;
    let mut components_name: HashSet<_> = ["lh", "fk", "ff", "mm"].into();

    let mut button_pushed = 0;
    loop {
        let mut signals_queue = VecDeque::new();
        signals_queue.push_back(vec![(
            "button".to_string(),
            "broadcaster".to_string(),
            Signal::LowPulse,
        )]);
        button_pushed += 1;

        while let Some(signals) = signals_queue.pop_front() {
            for (source, target, signal) in signals {
                if components_name.contains(source.as_str()) && matches!(signal, Signal::HighPulse)
                {
                    components_name.remove(source.as_str());
                    result = result.lcm(&button_pushed);
                    if components_name.is_empty() {
                        return Some(result);
                    }
                }
                if let Some(module) = modules.get_mut(target.as_str()) {
                    let outputs = module.signal(source.as_str(), signal);
                    signals_queue.push_back(
                        outputs
                            .into_iter()
                            .map(|(new_target, signal)| (target.clone(), new_target, signal))
                            .collect(),
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn another_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }
}
