# SMSG_SPELLLOGMISS

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L8).
```rust,ignore
smsg SMSG_SPELLLOGMISS = 0x024B {
    u32 id;
    Guid caster;
    u8 unknown1;
    u32 amount_of_targets;
    SpellLogMiss[amount_of_targets] targets;
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
| 0x04 | 4 / Little | u32 | id |  |  |
| 0x08 | 8 / Little | [Guid](../spec/packed-guid.md) | caster |  |  |
| 0x10 | 1 / - | u8 | unknown1 |  | cmangos/mangoszero: can be 0 or 1 |
| 0x11 | 4 / Little | u32 | amount_of_targets |  |  |
| 0x15 | ? / - | [SpellLogMiss](spelllogmiss.md)[amount_of_targets] | targets |  |  |

