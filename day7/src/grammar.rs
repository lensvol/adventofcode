use nom::{digit, alpha, multispace};
use std::str;
use std::fmt;

#[derive(Debug)]
pub enum Value<'a> {
    Number(u16),
    WireRef(&'a str),
}

#[derive(Debug)]
pub enum OpType {
    LoadConstant,
    Not,
    RShift,
    LShift,
    And,
    Or,
}

#[derive(Debug)]
pub struct Node<'a> {
    pub op: OpType,
    pub arg1: Value<'a>,
    pub arg2: Value<'a>,
}

#[derive(Debug)]
pub struct Link<'a> {
    pub input: Node<'a>,
    pub output: &'a str,
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Value::Number(n) => n.to_string(),
            Value::WireRef(s) => s.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for OpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OpType::Not => "NOT",
            OpType::And => "AND",
            OpType::Or => "OR",
            OpType::LShift => "RSHIFT",
            OpType::RShift => "LSHIFT",
            _ => "",
        };
        write!(f, "{}", s)
    }
}

impl<'a> fmt::Display for Link<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.input, self.output)
    }
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.op {
            OpType::LoadConstant => write!(f, "{}", self.arg1),
            OpType::Not => write!(f, "NOT {}", self.arg1),
            _ => write!(f, "{} {} {}", self.arg1, self.op, self.arg2),
        }
    }
}

fn make_wire_value<'a>(name: &'a str) -> Result<Value, i32> {
    Ok(Value::WireRef(name))
}

fn make_number<'a>(number: &'a str) -> Result<Value, i32> {
    Ok(Value::Number(number.parse::<u16>().unwrap()))
}

fn make_constant(v: Value) -> Result<Node, i32> {
    Ok(Node {
        op: OpType::LoadConstant,
        arg1: v,
        arg2: Value::Number(0),
    })
}

named!(
    wire_name<Value>,
    map_res!(
        map_res!(
            alpha,
            str::from_utf8
        ),
        make_wire_value
    )
);

named!(
    number<Value>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        make_number
    )
);

named!(
    value<Value>,
    alt!(number | wire_name)
);

named!(
    constant<Node>,
    map_res!(
        value,
        make_constant
    )
);

named!(
    not<Node>,
    chain!(
        tag!("NOT") ~
        multispace ~
        v: value,
        || { Node { op: OpType::Not, arg1: v, arg2: Value::Number(0) } }
    )
);

named!(
    operation<&str>,
    map_res!(
        alpha,
        str::from_utf8
    )
);

fn make_binary_op<'a>(operation: &str, arg1: Value<'a>, arg2: Value<'a>) -> Node<'a> {
    let op_type = match operation {
        "AND" => OpType::And,
        "RSHIFT" => OpType::RShift,
        "LSHIFT" => OpType::LShift,
        "OR" => OpType::Or,
        _ => panic!("Unknown operation: {}", operation),
    };

    Node {
        op: op_type,
        arg1: arg1,
        arg2: arg2,
    }
}

named!(
    binary_op<Node>,
    chain!(
        arg1: value ~
        multispace ~
        op: operation ~
        multispace ~
        arg2: value,
        || { make_binary_op(op, arg1, arg2) }
    )
);

named!(
    input<Node>,
    alt!(binary_op | not | constant)
);

named!(
    pub link<Link>,
    chain!(
        source: delimited!(opt!(multispace), input, opt!(multispace)) ~
        tag!("->") ~
        destination: delimited!(opt!(multispace), alpha, opt!(multispace)),
        || { Link { input: source, output: str::from_utf8(destination).unwrap() } }
    )
);

named!(
    pub instruction_kit<Vec<Link> >,
    many0!(link)
);
