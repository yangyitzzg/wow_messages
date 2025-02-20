# SMSG_SUPERCEDED_SPELL

## Client Version 1, Client Version 2

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm#L1).
```rust,ignore
smsg SMSG_SUPERCEDED_SPELL = 0x012C {
    u16 new_spell_id;
    u16 old_spell_id;
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
| 0x04 | 2 / Little | u16 | new_spell_id |  |  |
| 0x06 | 2 / Little | u16 | old_spell_id |  |  |

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm#L8).
```rust,ignore
smsg SMSG_SUPERCEDED_SPELL = 0x012C {
    u32 new;
    u32 old;
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
| 0x04 | 4 / Little | u32 | new |  |  |
| 0x08 | 4 / Little | u32 | old |  |  |

