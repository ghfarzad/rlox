use std::env;
use std::error::Error;

#[repr(u8)]
enum OpCode {
    OpConstant,
    OpReturn,
}

impl From<OpCode> for u8 {
    fn from(op_code: OpCode) -> u8 {
        op_code as u8
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            x if x == OpCode::OpConstant as u8 => Ok(OpCode::OpConstant),
            x if x == OpCode::OpReturn as u8 => Ok(OpCode::OpReturn),
            _ => Err(()),
        }
    }
}

struct ValueArray {
    code: Vec<f64>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        return ValueArray {
            code: Vec::new()
        }
    }

    pub fn read(&self, index: usize) -> f64 {
        self.code[index]
    }

    pub fn write(&mut self, value: f64) -> () {
        self.code.push(value);
    }

    pub fn count(&self) -> usize {
        self.code.len()
    }
}

struct Chunk {
    code: Vec<u8>,
    lines: Vec<u8>,
    constants: ValueArray
}

impl Chunk {
    pub fn new() -> Chunk {
        return Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new()
        }
    } 

    pub fn write(&mut self, byte: u8, line: u8) -> () {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.write(value);
        self.constants.count() - 1
    }
}

fn disassemble_chunk(chunk: &Chunk, name: &str) -> () {
    println!("== {} ==", name);

    let mut i = 0;
    while i < chunk.code.len() {
        i = i + disassemble_instruction(chunk, i);
    }
}

fn disassemble_constant(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let i = chunk.code[offset + 1];

    print!("{name} {offset:0>4} ", name = name, offset = offset);
    println!("{value}", value = chunk.constants.read(i.into()));

    return 2;
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:0>4} ", offset = offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ")
    } else {
        print!("{line:0>4} ", line = chunk.lines[offset]);
    }

    let instruction = chunk.code[offset];
    match instruction.try_into() {
        Ok(OpCode::OpConstant) => {
           return disassemble_constant("OpConstant", chunk, offset);
        },
        Ok(OpCode::OpReturn) => {
            println!("OpReturn");
            return 1;
        },
        Err(_) => {
            println!("Unknown OpCode {}", instruction);
            return 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let mut chunk = Chunk::new();

            let i = chunk.add_constant(1.2);

            let imaginary_line_number = 123;
            chunk.write(OpCode::OpConstant.into(), imaginary_line_number);
            chunk.write(i.try_into().unwrap(), imaginary_line_number);

            chunk.write(OpCode::OpReturn.into(),imaginary_line_number);

            disassemble_chunk(&chunk, "test_chunk");
        },
        _ => {
            println!("Usage: generate_ast<output directory>");
        }
    }

    Ok(())
}
