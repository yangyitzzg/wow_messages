# SMSG_SET_REST_START

## Client Version 1, Client Version 2

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/exp/smsg_set_rest_start.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_set_rest_start.wowm#L3).
```rust,ignore
smsg SMSG_SET_REST_START = 0x021E {
    u32 unknown1;
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
| 0x04 | 4 / Little | u32 | unknown1 |  | cmangos/mangoszero: unknown, may be rest state time or experience |

