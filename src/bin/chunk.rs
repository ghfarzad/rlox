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
    constants: ValueArray
}

impl Chunk {
    pub fn new() -> Chunk {
        return Chunk {
            code: Vec::new(),
            constants: ValueArray::new()
        }
    } 

    pub fn write(&mut self, byte: u8) -> () {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.write(value);
        self.constants.count() - 1
    }
}

fn disassemble_chunk(chunk: &Chunk, name: &str) -> () {
    println!("== {} ==", name);

    for (pos, e) in chunk.code.iter().enumerate() {
        disassemble_instruction(e, pos);
    }
}

fn disassemble_instruction(instruction: &u8, offset: usize) -> () {
    print!("{offset:0>4} ", offset = offset);

    let i = *instruction;
    match i.try_into() {
        Ok(OpCode::OpReturn) => { println!("OpReturn"); },
        Err(_) => { println!("Unknown OpCode {}", instruction) }
    }
}



fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let mut chunk = Chunk::new();
            chunk.write(OpCode::OpReturn.into());
            disassemble_chunk(&chunk, "test_chunk");
        },
        _ => {
            println!("Usage: generate_ast<output directory>");
        }
    }

    Ok(())
}
