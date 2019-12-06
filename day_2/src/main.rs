fn main() {
    let input: Vec<u32> = std::fs::read_to_string("./input.txt").unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    println!("input: {:?}", input);
    part_one(&input);

    part_two(&input);
}

#[derive(Debug)]
struct Cpu {
    memory: Vec<u32>,
    program_counter: usize,
    running: bool
}

impl Cpu {
    fn new(input: &Vec<u32>) -> Self {
        Cpu {
            memory: input.clone(),
            program_counter: 0,
            running: true
        }
    }

    fn read(&self, addr: usize) -> u32 {
        self.memory[addr]
    }

    fn write(&mut self, addr: usize, val: u32) {
        self.memory[addr] = val;
    }

    fn run(&mut self) {
        let pc = self.program_counter;
        let opcode = self.read(self.program_counter);
        println!("opcode: {}", opcode);
        match opcode {
            99 => self.running = false,
            1 | 2 => {
                let (x, y) = (self.read(pc + 1) as usize, self.read(pc + 2) as usize);
                let (x, y) = (self.read(x), self.read(y));
                let result;
                if opcode == 1 {
                    result = x + y;
                } else {
                    result = x * y;
                }
                let addr = self.read(pc + 3) as usize;
                self.write(addr, result);
            }
            _ => panic!("unrecognized opcode {}", opcode)
        }

        self.program_counter += 4;
    }
}

fn part_one(input: &Vec<u32>) {
    let mut cpu = Cpu::new(input);
    // restore program state
    cpu.write(1, 12);
    cpu.write(2, 2);

    while cpu.running {
        cpu.run();
    }

    println!("{}", cpu.read(0))
}

fn part_two(input: &Vec<u32>) {
    let const DESIRED = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = Cpu::new(input);
            cpu.write(1, noun);
            cpu.write(2, verb);

            while cpu.running {
                cpu.run();
            }

            if cpu.read(0) == DESIRED {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
