use smpl_core_common::Instruction;
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
//case!(multi_comment, "/* This is a\n * multiline\n * comment\n */", Ok(vec![]));
case!(nop_comment, "nop // This is a comment", Ok(vec![Instruction::nop()]));

case!(nop, "nop", Ok(vec![Instruction::nop()]));

case!(db, "db 0xF3", Ok(vec![Instruction::db(0xF3)]));
case!(db_multi, "db 0xF3, 0x37", Ok(vec![Instruction::db(0xF3), Instruction::db(0x37)]));
case!(db_err, "db 0xFFFF", Err(Error::NumberTooLarge(0xFFFF, "byte")));
case!(dw, "dw 0xF337", Ok(vec![Instruction::db(0x37), Instruction::db(0xF3)]));
