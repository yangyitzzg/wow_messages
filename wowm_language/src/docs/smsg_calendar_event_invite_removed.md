# SMSG_CALENDAR_EVENT_INVITE_REMOVED

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed.wowm#L1).
```rust,ignore
smsg SMSG_CALENDAR_EVENT_INVITE_REMOVED = 0x043B {
    PackedGuid invitee;
    Guid event_id;
    u32 flags;
    Bool show_alert;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | invitee |  |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | event_id |  |  |
| - | 4 / Little | u32 | flags |  |  |
| - | 1 / - | Bool | show_alert |  |  |

