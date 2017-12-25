use std::collections::HashMap;

type Register = char;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    R(Register),
    V(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Snd(Register),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Value),
    Jgz(Register, Value),
}

pub fn parse_instructions(text: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in text.lines() {
        let mut words = line.split_whitespace();
        let i = words.next().unwrap();
        match i {
            "set" => {
                let instruction = {
                    let r = words.next().unwrap().chars().next().unwrap();
                    let v = words.next().unwrap();

                    let parsed = v.parse::<i64>();
                    if let Ok(value) = parsed {
                        Instruction::Set(r, Value::V(value))
                    } else {
                        let v = v.chars().next().unwrap();
                        Instruction::Set(r, Value::R(v))
                    }
                };
                result.push(instruction);
            }
            "add" => {
                let instruction = {
                    let r = words.next().unwrap().chars().next().unwrap();
                    let v = words.next().unwrap();

                    let parsed = v.parse::<i64>();
                    if let Ok(value) = parsed {
                        Instruction::Add(r, Value::V(value))
                    } else {
                        let v = v.chars().next().unwrap();
                        Instruction::Add(r, Value::R(v))
                    }
                };
                result.push(instruction);
            }
            "mul" => {
                let instruction = {
                    let r = words.next().unwrap().chars().next().unwrap();
                    let v = words.next().unwrap();

                    let parsed = v.parse::<i64>();
                    if let Ok(value) = parsed {
                        Instruction::Mul(r, Value::V(value))
                    } else {
                        let v = v.chars().next().unwrap();
                        Instruction::Mul(r, Value::R(v))
                    }
                };
                result.push(instruction);
            }
            "mod" => {
                let instruction = {
                    let r = words.next().unwrap().chars().next().unwrap();
                    let v = words.next().unwrap();

                    let parsed = v.parse::<i64>();
                    if let Ok(value) = parsed {
                        Instruction::Mod(r, Value::V(value))
                    } else {
                        let v = v.chars().next().unwrap();
                        Instruction::Mod(r, Value::R(v))
                    }
                };
                result.push(instruction);
            }
            "jgz" => {
                let instruction = {
                    let r = words.next().unwrap().chars().next().unwrap();
                    let v = words.next().unwrap();

                    let parsed = v.parse::<i64>();
                    if let Ok(value) = parsed {
                        Instruction::Jgz(r, Value::V(value))
                    } else {
                        let v = v.chars().next().unwrap();
                        Instruction::Jgz(r, Value::R(v))
                    }
                };
                result.push(instruction);
            }
            "snd" => {
                let instruction = {
                    let r = words.next().unwrap().chars().next().unwrap();
                    Instruction::Snd(r)
                };
                result.push(instruction);
            }
            "rcv" => {
                let instruction = {
                    let r = words.next().unwrap();
                    let parsed = r.parse::<i64>();
                    if let Ok(value) = parsed {
                        Instruction::Rcv(Value::V(value))
                    } else {
                        let r = r.chars().next().unwrap();
                        Instruction::Rcv(Value::R(r))
                    }
                };
                result.push(instruction);
            }
            _ => panic!("Unhandled instruction: {}", i),
        }
    }
    result
}

#[derive(Default, Debug)]
pub struct State {
    pc: i64,
    last_sound: i64,
    registers: HashMap<char, i64>,
}

pub fn evaluate(instructions: &[Instruction]) -> i64 {
    use Instruction::*;

    let mut state = State::default();
    loop {
        let instruction = &instructions[state.pc as usize];
        match instruction {
            &Set(ref r, ref v) => {
                match *v {
                    Value::V(value) => {
                        *state.registers.entry(*r).or_insert(0) = value;
                    }
                    Value::R(name) => {
                        let value = state.registers[&name];
                        *state.registers.entry(*r).or_insert(0) = value;
                    }
                }
            }
            &Add(ref r, ref v) => {
                match *v {
                    Value::V(value) => {
                        *state.registers.entry(*r).or_insert(0) += value;
                    }
                    Value::R(name) => {
                        let value = state.registers[&name];
                        *state.registers.entry(*r).or_insert(0) += value;
                    }
                }
            }
            &Mul(ref r, ref v) => {
                match *v {
                    Value::V(value) => {
                        *state.registers.entry(*r).or_insert(0) *= value;
                    }
                    Value::R(name) => {
                        let value = state.registers[&name];
                        *state.registers.entry(*r).or_insert(0) *= value;
                    }
                }
            }
            &Mod(ref r, ref v) => {
                match *v {
                    Value::V(value) => {
                        *state.registers.entry(*r).or_insert(0) %= value;
                    }
                    Value::R(name) => {
                        let value = state.registers[&name];
                        *state.registers.entry(*r).or_insert(0) %= value;
                    }
                }
            }
            &Snd(ref r) => {
                state.last_sound = state.registers[&r];
            }
            &Rcv(ref r) => {
                match r {
                    &Value::V(value) => {
                        if value > 0 {
                            return state.last_sound;
                        }
                    }
                    &Value::R(name) => {
                        let value = state.registers[&name];
                        if value > 0 {
                            return state.last_sound;
                        }
                    }
                }
            }
            &Jgz(ref r, ref v) => {
                let value = state.registers[&r];
                if value > 0 {
                    match v {
                        &Value::V(vv) => {
                            state.pc += vv;
                            /* Do not increment the program counter */
                            continue;
                        }
                        &Value::R(rr) => {
                            let newvalue = state.registers[&rr];
                            state.pc += newvalue;
                        }
                    }
                }
            }
        }

        state.pc += 1;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions = parse_instructions(input);
    let value = evaluate(&instructions);
    println!("VALUE: {}", value);
}

#[cfg(test)]
mod test {
    use super::*;

    fn expected_instructions() -> Vec<Instruction> {
        use Instruction::*;
        vec![
            Set('a', Value::V(1)),
            Add('a', Value::V(2)),
            Mul('a', Value::R('a')),
            Mod('a', Value::V(5)),
            Snd('a'),
            Set('a', Value::V(0)),
            Rcv(Value::R('a')),
            Jgz('a', Value::V(-1)),
            Set('a', Value::V(1)),
            Jgz('a', Value::V(-2)),
        ]
    }

    #[test]
    fn test_parse_example() {
        let instructions = "set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2";

        let instructions = parse_instructions(instructions);
        assert_eq!(instructions, expected_instructions());
    }

    #[test]
    fn test_example_program() {
        let instructions = expected_instructions();
        let result = evaluate(&instructions);
        assert_eq!(result, 4);
    }
}
