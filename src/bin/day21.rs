use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Debug)]
enum Operand<'a> {
    Num(isize),
    Calc(Op, &'a str, &'a str),
}

// recursively evaluates operands
fn eval_op(operand: &Operand, operands: &HashMap<&str, Operand>) -> isize {
    return match operand {
        Operand::Num(val) => *val,
        Operand::Calc(op, lhs, rhs) => {
            let lhs_val = eval_op(&operands[lhs], operands);
            let rhs_val = eval_op(&operands[rhs], operands);
            match op {
                Op::Add => lhs_val + rhs_val,
                Op::Mul => lhs_val * rhs_val,
                Op::Sub => lhs_val - rhs_val,
                Op::Div => lhs_val / rhs_val,
            }
        }
    };
}

// return path from this node to human
fn node_to_humn<'a>(
    operand: &Operand<'a>,
    operands: &HashMap<&'a str, Operand<'a>>,
    name: &'a str,
) -> Option<Vec<&'a str>> {
    let mut result: Vec<&str> = vec![name];
    match operand {
        Operand::Num(_) => return None,
        Operand::Calc(op, lhs, rhs) => {
            if lhs == &"humn" || rhs == &"humn" {
                return Some(vec![name, "humn"]);
            }

            let lhs_op = &operands[lhs];
            let rhs_op = &operands[rhs];

            if let Some(v) = node_to_humn(lhs_op, operands, lhs) {
                result.append(&mut v.clone());
            } else if let Some(v) = node_to_humn(rhs_op, operands, rhs) {
                result.append(&mut v.clone());
            } else {
                return None;
            }
        }
    };
    return Some(result);
}

/// Starting from root, we know what the inverse calculation tracing the path from root to human
/// has to be equal to. At each step on the path, we can directly evaluate the child branch that
/// does not go to human, and then enact the inverse operation on the 'result' we want until
/// we reach human, at which point we will be left with the appropriate value that yields result
/// when the tree is evaluated in forward mode.
fn eval_op_inverse(
    operand: &Operand,
    operands: &HashMap<&str, Operand>,
    humn_path: &mut Vec<&str>,
    res: isize,
    name: &str,
) -> isize {
    if name == "humn" {
        return res;
    }

    let (op, lhs, rhs) = match operand {
        Operand::Calc(op, lhs, rhs) => (op, lhs, rhs),
        _ => unreachable!(),
    };
    let humn_left = lhs == &humn_path.pop().unwrap();
    let (to_solve, other) = if humn_left {
        // lhs("humn") . rhs = res  => lhs("humn") = res /. rhs
        // lhs("humn") - rhs = res => lhs("humn") = res + rhs
        // lhs("humn") + rhs = res => lhs("humn") = res - rhs
        // lhs("humn") * rhs = res => lhs("humn") = res / rhs
        // lhs("humn") / rhs = res => lhs("humn") = rhs * res
        (lhs, eval_op(&operands[rhs], &operands))
    } else {
        // lhs . rhs("humn") = res  =>   rhs("humn") = lhs /. res
        // lhs - rhs("humn") = res => rhs("humn") = lhs - res
        // lhs + rhs("humn") = res => rhs("humn") = res - rhs
        // lhs * rhs("humn") = res => rhs("humn") = res / lhs
        // lhs / rhs("humn") = res => rhs("humn") = lhs / res
        (rhs, eval_op(&operands[lhs], &operands))
    };

    match (op, humn_left) {
        (Op::Add, _) => eval_op_inverse(
            &operands[to_solve],
            &operands,
            humn_path,
            res - other,
            to_solve,
        ),
        (Op::Mul, _) => eval_op_inverse(
            &operands[to_solve],
            &operands,
            humn_path,
            res / other,
            to_solve,
        ),
        (Op::Sub, false) => eval_op_inverse(
            &operands[to_solve],
            &operands,
            humn_path,
            other - res,
            to_solve,
        ),
        (Op::Div, false) => eval_op_inverse(
            &operands[to_solve],
            &operands,
            humn_path,
            other / res,
            to_solve,
        ),
        (Op::Sub, true) => eval_op_inverse(
            &operands[to_solve],
            &operands,
            humn_path,
            other + res,
            to_solve,
        ),
        (Op::Div, true) => eval_op_inverse(
            &operands[to_solve],
            &operands,
            humn_path,
            other * res,
            to_solve,
        ),
    }
}

fn main() {
    // Init input reader
    let file = File::open(r".\src\data\day21_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    // populate hashmap of operands
    let mut operands: HashMap<&str, Operand> = HashMap::new();
    for line in all_lines.iter() {
        // construct all the operands
        let linesplit = line.split(" ").collect::<Vec<&str>>();
        if linesplit.len() > 2 {
            // Calc
            let op = match linesplit[2] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => unreachable!(),
            };
            let operand = Operand::Calc(op, linesplit[1], linesplit[3]);
            operands.insert(&linesplit[0][0..linesplit[0].len() - 1], operand);
        } else {
            let operand = Operand::Num(linesplit[1].parse::<isize>().unwrap());
            operands.insert(&linesplit[0][0..linesplit[0].len() - 1], operand);
        }
    }
    // evaluate root op, part 1
    println!("{}", eval_op(&operands["root"], &operands));

    // hardcode printing of root's two values
    println!(
        "{} {}",
        eval_op(&operands["wvvv"], &operands),
        eval_op(&operands["whqc"], &operands)
    );

    let mut humn_path = node_to_humn(&operands["root"], &operands, "root").unwrap();
    humn_path.reverse();
    println!("{:?}", humn_path);
    humn_path.pop(); // pop root off the path
    let (lhs, rhs) = match &operands["root"] {
        Operand::Calc(op, lhs, rhs) => (lhs, rhs),
        _ => unreachable!(),
    };
    if lhs == &humn_path.pop().unwrap() {
        let res = eval_op(&operands[rhs], &operands);
        println!(
            "{}",
            eval_op_inverse(&operands[lhs], &operands, &mut humn_path, res, lhs)
        );
    } else {
        let res = eval_op(&operands[rhs], &operands);
        println!(
            "{}",
            eval_op_inverse(&operands[lhs], &operands, &mut humn_path, res, rhs)
        );
    }
}
