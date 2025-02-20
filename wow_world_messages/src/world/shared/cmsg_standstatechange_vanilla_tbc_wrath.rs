use std::io::{Read, Write};

use wow_world_base::shared::unit_stand_state_vanilla_tbc_wrath::UnitStandState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm#L3):
/// ```text
/// cmsg CMSG_STANDSTATECHANGE = 0x0101 {
///     (u32)UnitStandState animation_state;
/// }
/// ```
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl crate::private::Sealed for CMSG_STANDSTATECHANGE {}
impl CMSG_STANDSTATECHANGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // animation_state: UnitStandState
        let animation_state = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            animation_state,
        })
    }

}

impl crate::Message for CMSG_STANDSTATECHANGE {
    const OPCODE: u32 = 0x0101;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_STANDSTATECHANGE {{").unwrap();
        // Members
        writeln!(s, "    animation_state = {};", self.animation_state.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 257_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "animation_state", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // animation_state: UnitStandState
        w.write_all(&u32::from(self.animation_state.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(257, "CMSG_STANDSTATECHANGE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_STANDSTATECHANGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_STANDSTATECHANGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_STANDSTATECHANGE {}

