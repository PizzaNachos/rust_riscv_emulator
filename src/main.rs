struct RiscVCpu{
    program_counter: u32,
    registers: [u32;32],
    program_memory: [u8; 0x2000],
}

impl RiscVCpu{
    fn read_instruction(&self) -> u32{
        let first = self.program_memory[self.program_counter as usize] as u32 ;
        let second = first << 8 | self.program_memory[self.program_counter as usize + 1] as u32 ;
        let third = second << 8 | self.program_memory[self.program_counter as usize + 2] as u32 ;
        third << 8 | self.program_memory[self.program_counter as usize + 3] as u32 
    }

    fn run(&mut self){
        loop{
            let instruction = self.read_instruction();
            if instruction == 0x0000 {
                break;
            };

            let opcode = instruction & 0x7f;
            println!("\nIn: {:032b}", instruction);
            println!("\tOpcode: {:08b}", opcode);

            match opcode{
                0b00010011 => self.l_type(instruction),
                0b00110011 => self.r_type(instruction),
                0b00000011 => self.load_type(instruction),
                0b00100011 => self.store_type(instruction),
                0b01101111 => self.jump_imm_link(instruction),
                0b01100111 => self.jump_imm_link_reg(instruction),
                _ => println!("Un-supported Instruction {:08b}", opcode),
            }

            self.program_counter += 4;
        }

    }
    fn load_program(&mut self, program_bytes: Vec<u8>) -> Result<(), ()>{
        if program_bytes.len() > 0x1000 {
            return Err(())
        }
        for (i,instruction) in program_bytes.iter().enumerate(){
            // println!("Adding Byte {:08b}", *instruction);
            self.program_memory[i] = *instruction;
        }
        Ok(())
    }
    fn new() -> RiscVCpu{
        RiscVCpu{
            program_counter: 0,
            registers: [0; 32],
            program_memory: [0; 0x2000],
        }
    }

    fn l_type(&mut self, instruction: u32){
        println!("\tl_type");
        let t = (instruction >> 12) & 0b111;
        let rd = (instruction >> 7) & 0x1f;
        let rs1 = (instruction >> 15) & 0x1f;
        let imm = instruction >> 20;

        match t {
            // ADDI
            0b000 => self.registers[rd as usize] = (self.registers[rs1 as usize] + imm),// as i32 * ((imm >> 31) as i32 * -1)) as u32,
            _ => println!("Not impliemted I type"),
        }
    }

    fn r_type(&mut self, instruction: u32){
        println!("\tr_type");

        let t = (instruction >> 12) & 0b111;
        let rd = (instruction >> 7) & 0x1f;
        let rs1 = (instruction >> 15) & 0x1f;
        let rs2 = (instruction >> 20) & 0x1f;
        let func = instruction >> 25;

        match t {
            // AND
            0b111 =>self.registers[rd as usize] = self.registers[rs1 as usize] & self.registers[rs2 as usize],
            // OR
            0b110 =>self.registers[rd as usize] = self.registers[rs1 as usize] | self.registers[rs2 as usize],
            // XOR
            0b100 =>self.registers[rd as usize] = self.registers[rs1 as usize] ^ self.registers[rs2 as usize],
            _ => println!("Not impliemted R type"),
        }
    }
    fn load_type(&mut self, instruction: u32){
        println!("\tLoad");

        let t = (instruction >> 12) & 0b111;
        let rd = (instruction >> 7) & 0x1f;
        let rs1 = (instruction >> 15) & 0x1f;
        let imm = instruction >> 20;

        match t {
            // LB
            0b000 => self.registers[rd as usize] = self.program_memory[rs1 as usize] as u32,
            // LH
            0b001 => {
                self.registers[rd as usize] = (self.program_memory[rs1 as usize] as u32) << 8 | (self.program_memory[rs1 as usize + 1] as u32)
             } ,
            // LW
            0b010 => {
                let reg = self.registers[rs1 as usize];
                let mut num = self.program_memory[reg as usize] as u32;
                num = num << 8;
                num = num | (self.program_memory[reg as usize + 1] as u32);
                num = num << 8;
                num = num | (self.program_memory[reg as usize + 2] as u32);
                num = num << 8;
                num = num | (self.program_memory[reg as usize + 3] as u32);
                self.registers[rd as usize] = num;
            },
            _ => println!("Not impliemted load type"),
        }
    }
    fn store_type(&mut self, instruction: u32){
        println!("\tStore");
        let t = (instruction >> 12) & 0b111;
        let rd = (instruction >> 7) & 0x1f;
        let source = (instruction >> 15) & 0x1f;
        let destination = (instruction >> 20) & 0x1f;
        let imm = instruction >> 25;
        match t {
            // SB
            0b000 => {
                let num = self.registers[source as usize];
                self.program_memory[destination as usize] = (num & 0xf) as u8;
            },
            // SH
            0b001 => {
                let num = self.registers[source as usize];
                self.program_memory[destination as usize] = (num & 0xf) as u8;
                self.program_memory[destination as usize + 1] = (num >> 8  & 0xf) as u8;             
            } ,
            // SW
            0b010 => {
                let num = self.registers[source as usize];
                let dest = self.registers[destination as usize];
                self.program_memory[dest as usize] = ((num >> 24) & 0xff) as u8;
                self.program_memory[dest as usize + 1] = ((num >> 16) & 0xff) as u8;
                self.program_memory[dest as usize + 2] = ((num >> 8)  & 0xff) as u8;
                self.program_memory[dest as usize + 3] = (num & 0xff) as u8;
                println!("Num:{}", num);
            },
            _ => println!("Not impliemted load type"),
        }
    }
    fn jump_imm_link(&mut self, instruction: u32){
        let rd = (instruction >> 7) & 0x1f;
        let imm = instruction >> 12 & 0x6ffff;
        self.registers[rd as usize] = self.program_counter;
        self.program_counter = imm;
        println!("\tJump and link");
        println!("\tImm: {}", imm);

    }
    fn jump_imm_link_reg(&mut self, instruction: u32){
        let rd = (instruction >> 7) & 0x1f;
        let t = (instruction >> 12) & 0x111;
        let rs1 = (instruction >> 15) & 0x1f;
        let imm = instruction >> 25;
        self.registers[rd as usize] = self.program_counter;
        self.program_counter = self.registers[rs1 as usize];

        println!("\tJump and link reg");
        println!("\tRs1: {:05b}", rs1);
    }
}


fn main() {
    let mut cpu = RiscVCpu::new();
    let program = read_ascii_file_to_vec("program_lean.txt".to_string());
    match cpu.load_program(program)
    {
        Ok(_) => {
            cpu.run();
        },
        Err(_) => println!("Program too large!")
    };
    
    println!("CPU Registers: ");
    for (i,r) in cpu.registers.iter().enumerate(){
        println!("R# {:05b} = {:032b}={}",i, *r, *r)
    }

    println!("Memory");
    let mut num : u32= 0;
    for (i,r) in cpu.program_memory.iter().enumerate(){
        if i % 4 == 0{
            if num != 0{
                println!("{}, {:032b}",i, num);
            }
            num = 0;
            continue;
        }
        num = (num << 8) | *r as u32; 
    }
}

fn read_ascii_file_to_vec(name: String) -> Vec<u8>{
    let mut r = vec![];

    if let Ok(file) = std::fs::read_to_string(name){
        let mut i = 0;
        let mut num : u8 = 0;
        for char in file.chars().into_iter(){
            match char{
                '0' => {
                    i += 1;
                    num = num << 1;
                },
                '1' => {
                    i += 1;
                    num = (num << 1) | 1;
                },

                _ => continue
            }

            if i % 8 == 0{
                r.push(num);
                i = 0;
                num = 0;
            }
        }
    };
    return r;
}
