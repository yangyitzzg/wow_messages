# CMSG_WORLD_STATE_UI_TIMER_UPDATE

## Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/cmsg_world_state_ui_timer_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_world_state_ui_timer_update.wowm#L3).
```rust,ignore
cmsg CMSG_WORLD_STATE_UI_TIMER_UPDATE = 0x04F6 {
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

This message has no fields in the body.

