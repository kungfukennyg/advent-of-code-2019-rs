fn main() {
    let input: Vec<i32> = std::fs::read_to_string("./input.txt").unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut cpu = Cpu::new(&input);
    while cpu.running {
        cpu.run();
    }
}

#[derive(Debug)]
struct Cpu {
    memory: Vec<i32>,
    program_counter: usize,
    running: bool
}

impl Cpu {
    fn new(input: &Vec<i32>) -> Self {
        Cpu {
            memory: input.clone(),
            program_counter: 0,
            running: true
        }
    }

    fn read(&self, addr: usize) -> i32 {
        self.memory[addr]
    }

    fn write(&mut self, addr: usize, val: i32) {
        self.memory[addr] = val;
    }

    fn run(&mut self) {
        let pc = self.program_counter;
        let instr = self.read(self.program_counter);
        let opcode = instr % 100;
        let (a, b, c) = decode_addressing_modes(instr);
        match opcode {
            1 | 2 | 5 | 6 | 7 | 8 => {
                let (x, y, z) = (self.read(pc + 1) as usize,
                              self.read(pc + 2) as usize, self.read(pc + 3) as usize);
                let (x, y) = (a.read(self, x), b.read(self, y));
                let mut result = std::i32::MIN;
                match opcode {
                    1 => {
                        result = x + y;
                        println!("ADD {} {} {}", x, y, z);
                        self.program_counter += 4;
                    }
                    2 => {
                        result = x * y;
                        println!("MUL {} {}", x, y);
                        self.program_counter += 4;
                    }
                    5 => {
                        println!("JMP NZERO {} {}", x, y);
                        if x != 0 {
                            self.program_counter = y as usize;
                        } else {
                            self.program_counter += 3;
                        }
                    }
                    6 => {
                        println!("JMP ZERO {} {}", x, y);
                        if x == 0 {
                            self.program_counter = y as usize;
                        } else {
                            self.program_counter += 3;
                        }
                    }
                    7 => {
                        println!("LESS {} {}", x, y);
                        result = if x < y { 1 } else { 0 };
                        self.program_counter += 4;
                    }
                    8 => {
                        println!("EQ {} {}", x, y);

                        result = if x == y { 1 } else { 0 };
                        self.program_counter += 4;
                    }
                    _ => panic!("unreachable")
                }

                if result != std::i32::MIN {
                    self.write(z, result);
                }
            }
            3 => {
                let input = 5;
                let addr = self.read(pc + 1) as usize;
                self.write(addr, input);
                self.program_counter += 2;
                println!("IN x{}", addr);
            }
            4 => {
                let addr = self.read(pc + 1) as usize;
                println!("OUT addr{}", addr);
                println!("{}", self.read(addr));
                self.program_counter += 2;
            }
            99 => self.running = false,
            _ => panic!("unrecognized opcode {}", opcode)
        }
    }
}

#[derive(Debug)]
enum AddressingMode {
    Immediate,
    Position
}

impl AddressingMode {
    fn from_num(num: i32) -> AddressingMode {
        match num {
            0 => AddressingMode::Position,
            1 => AddressingMode::Immediate,
            _ => panic!("unrecognized addresing mode {}", num)
        }
    }

    fn read(&self, cpu: &Cpu, param: usize) -> i32 {
        match self {
            AddressingMode::Immediate => param as i32,
            AddressingMode::Position => cpu.read(param)
        }
    }
}

fn decode_addressing_modes(instr: i32) -> (AddressingMode, AddressingMode, AddressingMode) {
    let a = AddressingMode::from_num((instr / 100) % 10);
    let b = AddressingMode::from_num((instr / 1000) % 10);
    let c = AddressingMode::from_num((instr / 10000) % 10);

    (a, b, c)
}