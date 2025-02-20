use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/cmsg_request_vehicle_switch_seat.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/cmsg_request_vehicle_switch_seat.wowm#L1):
/// ```text
/// cmsg CMSG_REQUEST_VEHICLE_SWITCH_SEAT = 0x0479 {
///     Guid vehicle;
///     u8 seat;
/// }
/// ```
pub struct CMSG_REQUEST_VEHICLE_SWITCH_SEAT {
    pub vehicle: Guid,
    pub seat: u8,
}

impl crate::private::Sealed for CMSG_REQUEST_VEHICLE_SWITCH_SEAT {}
impl CMSG_REQUEST_VEHICLE_SWITCH_SEAT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // vehicle: Guid
        let vehicle = crate::util::read_guid(&mut r)?;

        // seat: u8
        let seat = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vehicle,
            seat,
        })
    }

}

impl crate::Message for CMSG_REQUEST_VEHICLE_SWITCH_SEAT {
    const OPCODE: u32 = 0x0479;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_REQUEST_VEHICLE_SWITCH_SEAT {{").unwrap();
        // Members
        writeln!(s, "    vehicle = {};", self.vehicle.guid()).unwrap();
        writeln!(s, "    seat = {};", self.seat).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1145_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "vehicle", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vehicle: Guid
        w.write_all(&self.vehicle.guid().to_le_bytes())?;

        // seat: u8
        w.write_all(&self.seat.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1145, "CMSG_REQUEST_VEHICLE_SWITCH_SEAT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REQUEST_VEHICLE_SWITCH_SEAT {}

