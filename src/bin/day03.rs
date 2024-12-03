use advent_of_code_2024::load_input;

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

impl Instruction {
    fn value(&self) -> u32 {
        match self {
            Instruction::Mul(lhs, rhs) => lhs * rhs,
            Instruction::Do => 0,
            Instruction::Dont => 0,
        }
    }
}

type ParseResult<'src, T> = Result<(&'src str, T), &'src str>;

fn fixed_string<'s>(source: &'s str, expected: &'static str) -> ParseResult<'s, ()> {
    if source.starts_with(expected) {
        Ok((&source[expected.len()..], ()))
    } else {
        Err(source)
    }
}

fn number(source: &str) -> ParseResult<u32> {
    let num: String = source.chars().take_while(|ch| ch.is_numeric()).collect();

    if num.is_empty() {
        Err(source)
    } else {
        let value: u32 = num.parse().unwrap();
        Ok((&source[num.len()..], value))
    }
}

const IDENTS: [&str; 3] = ["mul", "don't", "do"];

fn identifier(source: &str) -> ParseResult<&str> {
    for ident in IDENTS {
        if source.starts_with(ident) {
            return Ok((&source[ident.len()..], ident));
        }
    }

    Err(source)
}

fn parse_call(source: &str) -> ParseResult<Instruction> {
    let (advanced, ident) = identifier(source).or(Err(source))?;
    let (advanced, _) = fixed_string(advanced, "(").or(Err(source))?;

    let (advanced, instruction) = match ident {
        "mul" => {
            let (advanced, lhs) = number(advanced).or(Err(source))?;
            let (advanced, _) = fixed_string(advanced, ",").or(Err(source))?;
            let (advanced, rhs) = number(advanced).or(Err(source))?;
            (advanced, Instruction::Mul(lhs, rhs))
        }
        "don't" => (advanced, Instruction::Dont),
        "do" => (advanced, Instruction::Do),
        _ => return Err(source),
    };

    let (advanced, _) = fixed_string(advanced, ")").or(Err(source))?;

    Ok((advanced, instruction))
}

fn main() {
    let input = load_input();
    let mut source = input.as_str();

    let mut instructions: Vec<Instruction> = Vec::new();

    loop {
        if source.len() == 0 {
            break;
        }

        match parse_call(source) {
            Ok((advanced, instruction)) => {
                instructions.push(instruction);
                source = advanced;
            }
            Err(advanced) => {
                source = &advanced[1..];
            }
        }
    }

    let total: u32 = instructions.iter().map(|ins| ins.value()).sum();

    println!("Part 1: {total}");

    let total: u32 = instructions
        .iter()
        .fold((true, 0), |(enable, acc), ins| match ins {
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
            Instruction::Mul(_, _) => {
                let value = if enable { ins.value() } else { 0 };
                (enable, value + acc)
            }
        })
        .1;

    println!("Part 2: {total}");
}
