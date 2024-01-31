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

case!(identifiers, "foo: mov bar, r0\nbar: mov rel foo, r1", Ok((
    vec![
        Instruction::movc2r(Value::word(4), Register::r0()).unwrap(),
        Instruction::movc2r(Value::word(-4i16 as u16), Register::r1()).unwrap()
    ],
    {
        let mut identifiers = HashMap::new();
        for (ident, offset) in vec![("foo", 0), ("bar", 4)].into_iter() {
            identifiers.insert(ident.to_string(), offset);
        }
        identifiers
    }
)));

case!(nop, "nop", Ok((vec![Instruction::nop()], HashMap::new())));

case!(db, "db 0xF3", Ok((vec![Instruction::db(0xF3)], HashMap::new())));
case!(db_multi, "db 0xF3, 0x37", Ok((vec![Instruction::db(0xF3), Instruction::db(0x37)], HashMap::new())));
case!(db_err, "db 0xFFFF", Err(Error::NumberTooLarge(0xFFFF, "byte")));
case!(dw, "dw 0xF337", Ok((vec![Instruction::db(0x37), Instruction::db(0xF3)], HashMap::new())));

case!(movc2r_byte, "mov 0xF3, rb0", Ok((vec![Instruction::movc2r(Value::byte(0xF3), Register::rb0()).unwrap()], HashMap::new())));
case!(movc2r_word, "mov 0xF337, r1", Ok((vec![Instruction::movc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));

case!(movr2r_byte, "mov rb2, rb3", Ok((vec![Instruction::movr2r(Register::rb2(), Register::rb3()).unwrap()], HashMap::new())));
case!(movr2r_word, "mov r4, r5", Ok((vec![Instruction::movr2r(Register::r4(), Register::r5()).unwrap()], HashMap::new())));

case!(movm2r, "mov [r6], rb7", Ok((vec![Instruction::movm2r(Register::r6(), Register::rb7()).unwrap()], HashMap::new())));
case!(movr2m, "mov rb8, [r9]", Ok((vec![Instruction::movr2m(Register::rb8(), Register::r9()).unwrap()], HashMap::new())));

case!(addc2r_byte, "add 0xF3, rb11", Ok((vec![Instruction::addc2r(Value::byte(0xF3), Register::rb11()).unwrap()], HashMap::new())));
case!(addc2r_word, "add 0xF337, r1", Ok((vec![Instruction::addc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));

case!(addr2r_byte, "add rb10, rb11", Ok((vec![Instruction::addr2r(Register::rb10(), Register::rb11()).unwrap()], HashMap::new())));
case!(addr2r_word, "add r0, r1", Ok((vec![Instruction::addr2r(Register::r0(), Register::r1()).unwrap()], HashMap::new())));

case!(subc2r_byte, "sub 0xF3, rb11", Ok((vec![Instruction::subc2r(Value::byte(0xF3), Register::rb11()).unwrap()], HashMap::new())));
case!(subc2r_word, "sub 0xF337, r1", Ok((vec![Instruction::subc2r(Value::word(0xF337), Register::r1()).unwrap()], HashMap::new())));

case!(subr2r_byte, "sub rb0, rb1", Ok((vec![Instruction::subr2r(Register::rb0(), Register::rb1()).unwrap()], HashMap::new())));
case!(subr2r_word, "sub r0, r1", Ok((vec![Instruction::subr2r(Register::r0(), Register::r1()).unwrap()], HashMap::new())));

case!(jmp, "jmp r0", Ok((vec![Instruction::jmp(Register::r0()).unwrap()], HashMap::new())));
