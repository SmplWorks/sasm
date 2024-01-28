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

case!(empty, "", Ok(vec![]));
case!(comment, "// This is a comment", Ok(vec![]));
case!(multi_comment, "/* This is a\n * multiline\n * comment\n */", Ok(vec![]));
case!(nop_comment, "nop // This is a comment", Ok(vec![Instruction::nop()]));

case!(nop, "nop", Ok(vec![Instruction::nop()]));

case!(db, "db 0xF3", Ok(vec![Instruction::db(0xF3)]));
case!(db_multi, "db 0xF3, 0x37", Ok(vec![Instruction::db(0xF3), Instruction::db(0x37)]));
case!(db_err, "db 0xFFFF", Err(Error::NumberTooLarge(0xFFFF, "byte")));
case!(dw, "dw 0xF337", Ok(vec![Instruction::db(0x37), Instruction::db(0xF3)]));

case!(movc2r_byte, "mov 0xF3, rb0", Ok(vec![Instruction::movc2r(Value::byte(0xF3), Register::rb0()).unwrap()]));
case!(movc2r_word, "mov 0xF337, r1", Ok(vec![Instruction::movc2r(Value::word(0xF337), Register::r1()).unwrap()]));

case!(movr2r_byte, "mov rb2, rb3", Ok(vec![Instruction::movr2r(Register::rb2(), Register::rb3()).unwrap()]));
case!(movr2r_word, "mov r4, r5", Ok(vec![Instruction::movr2r(Register::r4(), Register::r5()).unwrap()]));

case!(movm2r, "mov [r6], rb7", Ok(vec![Instruction::movm2r(Register::r6(), Register::rb7()).unwrap()]));
case!(movr2m, "mov rb8, [r9]", Ok(vec![Instruction::movr2m(Register::rb8(), Register::r9()).unwrap()]));

case!(add_byte, "add rb10, rb11", Ok(vec![Instruction::add(Register::rb10(), Register::rb11()).unwrap()]));
case!(add_word, "add r0, r1", Ok(vec![Instruction::add(Register::r0(), Register::r1()).unwrap()]));

case!(sub_byte, "sub rb0, rb1", Ok(vec![Instruction::sub(Register::rb0(), Register::rb1()).unwrap()]));
case!(sub_word, "sub r0, r1", Ok(vec![Instruction::sub(Register::r0(), Register::r1()).unwrap()]));
