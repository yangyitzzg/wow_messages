# CMSG_CALENDAR_COPY_EVENT

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/calendar/cmsg_calendar_copy_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_copy_event.wowm#L1).
```rust,ignore
cmsg CMSG_CALENDAR_COPY_EVENT = 0x0430 {
    Guid event;
    Guid invite_id;
    DateTime time;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | event |  |  |
| 0x0E | 8 / Little | [Guid](../spec/packed-guid.md) | invite_id |  |  |
| 0x16 | 4 / Little | DateTime | time |  |  |

