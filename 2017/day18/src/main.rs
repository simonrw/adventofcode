type Register = char;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    R(Register),
    V(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Snd(Register),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Register),
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

                    let parsed = v.parse::<i32>();
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

                    let parsed = v.parse::<i32>();
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

                    let parsed = v.parse::<i32>();
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

                    let parsed = v.parse::<i32>();
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

                    let parsed = v.parse::<i32>();
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
                    let r = words.next().unwrap().chars().next().unwrap();
                    Instruction::Rcv(r)
                };
                result.push(instruction);
            }
            _ => panic!("Unhandled instruction: {}", i),
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_example() {
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

        use Instruction::*;
        assert_eq!(
            instructions,
            vec![
                Set('a', Value::V(1)),
                Add('a', Value::V(2)),
                Mul('a', Value::R('a')),
                Mod('a', Value::V(5)),
                Snd('a'),
                Set('a', Value::V(0)),
                Rcv('a'),
                Jgz('a', Value::V(-1)),
                Set('a', Value::V(1)),
                Jgz('a', Value::V(-2)),
            ]
        );
    }
}
