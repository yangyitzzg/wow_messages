# SMSG_UPDATE_INSTANCE_OWNERSHIP

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm#L3).
```rust,ignore
smsg SMSG_UPDATE_INSTANCE_OWNERSHIP = 0x032B {
    Bool32 player_is_saved_to_a_raid;
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
| 0x04 | 4 / Little | Bool32 | player_is_saved_to_a_raid |  |  |

