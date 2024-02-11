use std::collections::HashMap;

use smpl_core_common::{Instruction, Register, Value};
use crate::{parse, utils::Error};

macro_rules! case {
    ($ident:ident, $code:literal, $result:expr) => {
        #[test]
        fn $ident() {
            assert_eq!(parse($code), $result);
        }
    };
}

case!(empty, "", Ok((vec![], HashMap::new())));
case!(comment, "// This is a comment", Ok((vec![], HashMap::new())));
case!(multi_comment, "/* This is a\n * multiline\n * comment\n */", Ok((vec![], HashMap::new())));
case!(nop_comment, "nop // This is a comment", Ok((vec![Instruction::nop()], HashMap::new())));

case!(identifiers_tmp, "foo: mov bar, r0\nbar: mov foo, r1", Ok((
    vec![
        Instruction::movc2r(Value::word(4), Register::r0()).unwrap(),
        Instruction::movc2r(Value::word(0), Register::r1()).unwrap()
    ],
    {
        let mut identifiers = HashMap::new();
        for (ident, offset) in vec![("foo", 0), ("bar", 4)].into_iter() {
            identifiers.insert(ident.to_string(), offset);
        }
        identifiers
    }
)));

/* TODO: Leaving it for a later reworking
case!(labels, "l0: mov l1, r0\nl1: mov [l1], r1\nl2: mov r2, [l2]\nl3: ajmp l3\nl4: jmp l4\nl5: call l5\n", Ok((
    vec![
    ],
    {
        let mut identifiers = HashMap::new();
        for (ident, offset) in vec![("l0", 0), ("l1", 4), ("l2", 8), ("l3", 12), ("l4", 16)].into_iter() {
            identifiers.insert(ident.to_string(), offset);
        }
        identifiers
    }
)));
*/

case!(nop, "nop", Ok((vec![Instruction::nop()], HashMap::new())));

case!(db, "db 0xF3", Ok((vec![Instruction::db(0xF3)], HashMap::new())));
case!(db_multi, "db 0xF3, 0x37", Ok((vec![Instruction::db(0xF3), Instruction::db(0x37)], HashMap::new())));
case!(db_err, "db 0xFFFF", Err(Error::NumberTooLarge(0xFFFF, "byte")));
case!(dw, "dw 0xF337", Ok((vec![Instruction::db(0x37), Instruction::db(0xF3)], HashMap::new())));

case!(movc2r_byte, "mov 0xF3, rb0", Ok((vec![Instruction::movc2r(Value::byte(0xF3), Register::rb0()).unwrap()], HashMap::new())));
case!(movc2r_word, "mov 0xF337, r1", Ok((vec![Instruction::movc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));

case!(movr2r_byte, "mov rb2, rb3", Ok((vec![Instruction::movr2r(Register::rb2(), Register::rb3()).unwrap()], HashMap::new())));
case!(movr2r_word, "mov r4, r5", Ok((vec![Instruction::movr2r(Register::r4(), Register::r5()).unwrap()], HashMap::new())));

case!(movm2r_byte, "mov [r6], rb7", Ok((vec![Instruction::movm2r(Register::r6(), Register::rb7()).unwrap()], HashMap::new())));
case!(movm2r_word, "mov [r6], r7", Ok((vec![Instruction::movm2r(Register::r6(), Register::r7()).unwrap()], HashMap::new())));
case!(movr2m_byte, "mov rb8, [r9]", Ok((vec![Instruction::movr2m(Register::rb8(), Register::r9()).unwrap()], HashMap::new())));
case!(movr2m_word, "mov r8, [r9]", Ok((vec![Instruction::movr2m(Register::r8(), Register::r9()).unwrap()], HashMap::new())));

case!(push, "push r0", Ok((vec![Instruction::push(Register::r0()).unwrap()], HashMap::new())));
case!(pop, "pop r0", Ok((vec![Instruction::pop(Register::r0()).unwrap()], HashMap::new())));

case!(addc2r_byte, "add 0xF3, rb1", Ok((vec![Instruction::addc2r(Value::byte(0xF3), Register::rb1()).unwrap()], HashMap::new())));
case!(addc2r_word, "add 0xF337, r1", Ok((vec![Instruction::addc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));
case!(addr2r_byte, "add rb0, rb1", Ok((vec![Instruction::addr2r(Register::rb0(), Register::rb1()).unwrap()], HashMap::new())));
case!(addr2r_word, "add r0, r1", Ok((vec![Instruction::addr2r(Register::r0(), Register::r1()).unwrap()], HashMap::new())));

case!(subc2r_byte, "sub 0xF3, rb1", Ok((vec![Instruction::subc2r(Value::byte(0xF3), Register::rb1()).unwrap()], HashMap::new())));
case!(subc2r_word, "sub 0xF337, r1", Ok((vec![Instruction::subc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));
case!(subr2r_byte, "sub rb0, rb1", Ok((vec![Instruction::subr2r(Register::rb0(), Register::rb1()).unwrap()], HashMap::new())));
case!(subr2r_word, "sub r0, r1", Ok((vec![Instruction::subr2r(Register::r0(), Register::r1()).unwrap()], HashMap::new())));

case!(not_byte, "not rb0", Ok((vec![Instruction::not(Register::rb0()).unwrap()], HashMap::new())));
case!(not_word, "not r0", Ok((vec![Instruction::not(Register::r0()).unwrap()], HashMap::new())));

case!(andc2r_byte, "and 0xF3, rb1", Ok((vec![Instruction::andc2r(Value::byte(0xF3), Register::rb1()).unwrap()], HashMap::new())));
case!(andc2r_word, "and 0xF337, r1", Ok((vec![Instruction::andc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));
case!(andr2r_byte, "and rb0, rb1", Ok((vec![Instruction::andr2r(Register::rb0(), Register::rb1()).unwrap()], HashMap::new())));
case!(andr2r_word, "and r0, r1", Ok((vec![Instruction::andr2r(Register::r0(), Register::r1()).unwrap()], HashMap::new())));

case!(orc2r_byte, "or 0xF3, rb1", Ok((vec![Instruction::orc2r(Value::byte(0xF3), Register::rb1()).unwrap()], HashMap::new())));
case!(orc2r_word, "or 0xF337, r1", Ok((vec![Instruction::orc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));
case!(orr2r_byte, "or rb0, rb1", Ok((vec![Instruction::orr2r(Register::rb0(), Register::rb1()).unwrap()], HashMap::new())));
case!(orr2r_word, "or r0, r1", Ok((vec![Instruction::orr2r(Register::r0(), Register::r1()).unwrap()], HashMap::new())));

#[test]
fn shl() {
    for i in 1..=8 {
        assert_eq!(parse(&format!("shl {i}, rb0")), Ok((vec![Instruction::shl(Value::byte(i), Register::rb0()).unwrap()], HashMap::new())));
    }

    for i in 1..=16 {
        assert_eq!(parse(&format!("shl {i}, r0")), Ok((vec![Instruction::shl(Value::word(i), Register::r0()).unwrap()], HashMap::new())));
    }
}

#[test]
fn shr() {
    for i in 1..=8 {
        assert_eq!(parse(&format!("shr {i}, rb0")), Ok((vec![Instruction::shr(Value::byte(i), Register::rb0()).unwrap()], HashMap::new())));
    }

    for i in 1..=16 {
        assert_eq!(parse(&format!("shr {i}, r0")), Ok((vec![Instruction::shr(Value::word(i), Register::r0()).unwrap()], HashMap::new())));
    }
}

#[test]
fn shre() {
    for i in 1..=8 {
        assert_eq!(parse(&format!("shre {i}, rb0")), Ok((vec![Instruction::shre(Value::byte(i), Register::rb0()).unwrap()], HashMap::new())));
    }

    for i in 1..=16 {
        assert_eq!(parse(&format!("shre {i}, r0")), Ok((vec![Instruction::shre(Value::word(i), Register::r0()).unwrap()], HashMap::new())));
    }
}

case!(cmpc2r_byte, "cmp 0xF3, rb1", Ok((vec![Instruction::cmpc2r(Value::byte(0xF3), Register::rb1()).unwrap()], HashMap::new())));
case!(cmpc2r_word, "cmp 0xF337, r1", Ok((vec![Instruction::cmpc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));
case!(cmpr2r_byte, "cmp rb0, rb1", Ok((vec![Instruction::cmpr2r(Register::rb0(), Register::rb1()).unwrap()], HashMap::new())));
case!(cmpr2r_word, "cmp r0, r1", Ok((vec![Instruction::cmpr2r(Register::r0(), Register::r1()).unwrap()], HashMap::new())));

case!(ajmp, "ajmp r0", Ok((vec![Instruction::ajmp(Register::r0()).unwrap()], HashMap::new())));
case!(jmp, "jmp r0", Ok((vec![Instruction::jmp(Register::r0()).unwrap()], HashMap::new())));
case!(jeq, "jeq r0", Ok((vec![Instruction::jeq(Register::r0()).unwrap()], HashMap::new())));
case!(jneq, "jneq r0", Ok((vec![Instruction::jneq(Register::r0()).unwrap()], HashMap::new())));
case!(jlt, "jlt r0", Ok((vec![Instruction::jlt(Register::r0()).unwrap()], HashMap::new())));
case!(jgt, "jgt r0", Ok((vec![Instruction::jgt(Register::r0()).unwrap()], HashMap::new())));
case!(jleq, "jleq r0", Ok((vec![Instruction::jleq(Register::r0()).unwrap()], HashMap::new())));
case!(jgeq, "jgeq r0", Ok((vec![Instruction::jgeq(Register::r0()).unwrap()], HashMap::new())));
case!(jo, "jo r0", Ok((vec![Instruction::jo(Register::r0()).unwrap()], HashMap::new())));
case!(jno, "jno r0", Ok((vec![Instruction::jno(Register::r0()).unwrap()], HashMap::new())));
case!(callc, "call 0xF337", Ok((vec![Instruction::callc(Value::word(0xF337)).unwrap()], HashMap::new())));
case!(callr, "call r0", Ok((vec![Instruction::callr(Register::r0()).unwrap()], HashMap::new())));

case!(int, "int r0", Ok((vec![Instruction::int(Register::r0()).unwrap()], HashMap::new())));
case!(sti, "sti r0", Ok((vec![Instruction::sti(Register::r0()).unwrap()], HashMap::new())));
case!(cli, "cli", Ok((vec![Instruction::cli()], HashMap::new())));
