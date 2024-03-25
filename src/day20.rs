use std::collections::{HashMap, VecDeque};

use crate::{read_file_to_string, SolveAdvent};

pub struct Day20;

impl SolveAdvent for Day20 {
    fn solve_part1(path_to_file: &str) {
        let input_file = read_file_to_string(path_to_file);
        let pulse_modules = PulseModule::from_input_file(&input_file);
        let mut module_map = pulse_modules
            .into_iter()
            .map(|module| (module.name.clone(), module))
            .collect::<HashMap<_, _>>();
        let mut total_low_pulses: usize = 0;
        let mut total_high_pulses: usize = 0;
        let total_button_presses = 1000;
        for _button_press in 0..total_button_presses {
            let (low_pulse_count, high_pulse_count) = invoke_pulse_loop_via_button(&mut module_map);
            total_high_pulses += high_pulse_count;
            total_low_pulses += low_pulse_count;
        }
        println!("After {total_button_presses} button presses, there were {total_low_pulses} low pulses and {total_high_pulses} high pulses");
        println!(
            "Multiplied low & high pulse counts together gives {}",
            total_low_pulses * total_high_pulses
        );
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
    }
}

///A pulses are either High or Low.
#[derive(Debug, Copy, Clone)]
enum PulseType {
    High,
    Low,
}

fn invoke_pulse_loop_via_button(module_map: &mut HashMap<String, PulseModule>) -> (usize, usize) {
    //! Invoke a single run of the modules. The button is pressend once to initiate the sequence by passing a
    //! Low pulse to the broadcaster.
    let mut pulse_message_queue = VecDeque::new();
    pulse_message_queue.push_back(PulseMessage {
        sender: "button".to_string(),
        state: PulseType::Low,
        recipients: vec!["broadcaster".to_owned()],
    });
    let mut low_pulses_sent = 0;
    let mut high_pulses_sent = 0;

    while let Some(pulse_message) = pulse_message_queue.pop_front() {
        match pulse_message.state {
            PulseType::High => {
                high_pulses_sent += pulse_message.recipients.len();
            }
            PulseType::Low => low_pulses_sent += pulse_message.recipients.len(),
        };
        for recipient_name in pulse_message.recipients.iter() {
            if let Some(module) = module_map.get_mut(recipient_name) {
                //For each recipient of the message, react to the message. If a new pulse message needs to be
                //sent out, but it at the back of the message queue.
                if let Some(next_pulse_module) = module.react_to_new_pulse(&pulse_message) {
                    pulse_message_queue.push_back(next_pulse_module);
                }
            }
        }
    }
    (low_pulses_sent, high_pulses_sent)
}

///A pulse message, which can be either
/// 'high': represented as a 1,
/// or 'low': represented as a 0, and the
/// recipient modules.
#[derive(Debug)]
struct PulseMessage {
    ///The PulseModule that sent the message
    sender: String,
    ///Is the pulse high or low?
    state: PulseType,
    ///The list of recipients of the message.
    recipients: Vec<String>,
}

///The 3 types of modules in the problem,
/// disregarding the button to initiate the whole
/// chain.
#[derive(Debug, Clone)]
enum PulseModuleKind {
    ///FlipFlops maintain a state of either 1 or 0. This affect whether
    /// they send out a low or high pulse upon receiving a message.
    FlipFlop { state: PulseType },
    ///Conjunctions maintain a memory of the last input of each input
    /// module. `pulse_memory` is a map of the Input Module Name to the last input
    /// from that input module.
    Conjunction {
        pulse_memory: HashMap<String, PulseType>,
    },
    ///Broadcasters do not to maintain any extra state.
    Broadcaster,
}

///Represents a single pulse module.
#[derive(Debug, Clone)]
struct PulseModule {
    ///The name of the module, for example `a`
    name: String,
    ///The names of the modules this module sends messages to
    destination_modules: Vec<String>,
    ///The type of module this is.
    kind: PulseModuleKind,
}

impl PulseModule {
    fn react_to_new_pulse(&mut self, message: &PulseMessage) -> Option<PulseMessage> {
        //! Given a new pulse passed in, optionally build a new message to be sent out in response.
        if !message.recipients.contains(&self.name) {
            return None;
        }
        match &mut self.kind {
            PulseModuleKind::Broadcaster => {
                //The broadcaster module passes the input pulse value to
                //every module in its destination module list.
                Some(PulseMessage {
                    sender: self.name.clone(),
                    state: message.state,
                    recipients: self.destination_modules.clone(),
                })
            }
            PulseModuleKind::Conjunction { pulse_memory } => {
                //The conjunction module first updates its memory state of the input to the new input.
                if let Some(memory_state) = pulse_memory.get_mut(&message.sender) {
                    *memory_state = message.state;
                }
                let inputs_all_high = pulse_memory.values().all(|memory_state| {
                    if let PulseType::High = memory_state {
                        return true;
                    }
                    false
                });
                //If all of the inputs in the conjunction history are high, then a low pulse
                //is sent out, otherwise a low pulse.
                let pulse_type_to_send = if inputs_all_high {
                    PulseType::Low
                } else {
                    PulseType::High
                };
                Some(PulseMessage {
                    sender: self.name.clone(),
                    state: pulse_type_to_send,
                    recipients: self.destination_modules.clone(),
                })
            }
            PulseModuleKind::FlipFlop { state } => {
                //If a high pulse is received, do nothing.
                if let PulseType::High = message.state {
                    return None;
                }
                //Otherwise, flip the state and send out the new state.
                match state {
                    PulseType::High => *state = PulseType::Low,
                    PulseType::Low => *state = PulseType::High,
                };
                Some(PulseMessage {
                    sender: self.name.clone(),
                    state: *state,
                    recipients: self.destination_modules.clone(),
                })
            }
        }
    }
    fn from_input_file(input_file: &str) -> Vec<PulseModule> {
        //! Build out the `PulseModules` from the input file.
        let mut pulse_modules = input_file
            .lines()
            .map(PulseModule::from_line)
            .collect::<Vec<_>>();
        //At this point, the problem is that Conjunction modules have not had their `pulse_memory`
        //map filled to contain all of the inputs to said conjunction module intialized to a Low Pulse.
        let mut destination_map = HashMap::new();
        for pulse_module in pulse_modules.iter() {
            destination_map.insert(
                pulse_module.name.clone(),
                pulse_module.destination_modules.clone(),
            );
        }

        for pulse_module in pulse_modules.iter_mut() {
            if let PulseModuleKind::Conjunction { pulse_memory } = &mut pulse_module.kind {
                for (module_name, module_dests) in destination_map.iter() {
                    //For every conjunction module, any modules that list this conjunction module in their destination
                    //list should have the input module's name added to the conjunction modules pulse_memory map initialized to
                    //a Low pulse.
                    if module_dests.contains(&pulse_module.name) {
                        pulse_memory.insert(module_name.clone(), PulseType::Low);
                    }
                }
            }
        }
        pulse_modules
    }
    fn from_line(line: &str) -> PulseModule {
        //! Construct a `PulseModule` from a line in the input file. The initialization
        //! for Conjunction modules in incomplete, as explained in the `from_input_file` method.
        let mut line_split = line.split("->");
        let module_name = line_split.next().unwrap().trim();
        let destination_modules = line_split
            .next()
            .unwrap()
            .trim()
            .replace(' ', "")
            .split(',')
            .map(|segment| segment.to_owned())
            .collect::<Vec<_>>();
        if module_name == "broadcaster" {
            PulseModule {
                name: module_name.to_owned(),
                destination_modules,
                kind: PulseModuleKind::Broadcaster,
            }
        } else if module_name.starts_with('%') {
            PulseModule {
                name: module_name.replace('%', ""),
                destination_modules,
                kind: PulseModuleKind::FlipFlop {
                    state: PulseType::Low,
                },
            }
        } else {
            PulseModule {
                name: module_name.replace('&', ""),
                destination_modules,
                kind: PulseModuleKind::Conjunction {
                    pulse_memory: HashMap::new(),
                },
            }
        }
    }
}
