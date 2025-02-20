# SMSG_CAST_RESULT

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_cast_result.wowm:449`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cast_result.wowm#L449).
```rust,ignore
smsg SMSG_CAST_RESULT = 0x0130 {
    u32 spell;
    SimpleSpellCastResult result;
    if (result != FAILURE) {
        CastFailureReason reason;
        if (reason == REQUIRES_SPELL_FOCUS) {
            u32 required_spell_focus;
        }
        else if (reason == REQUIRES_AREA) {
            Area area;
        }
        else if (reason == EQUIPPED_ITEM_CLASS) {
            u32 equipped_item_class;
            u32 equipped_item_subclass_mask;
            u32 equipped_item_inventory_type_mask;
        }
    }
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
| 0x04 | 4 / Little | u32 | spell |  |  |
| 0x08 | 1 / - | [SimpleSpellCastResult](simplespellcastresult.md) | result |  |  |

If result is not equal to `FAILURE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x09 | 1 / - | [CastFailureReason](castfailurereason.md) | reason |  |  |

If reason is equal to `REQUIRES_SPELL_FOCUS`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x0A | 4 / Little | u32 | required_spell_focus |  |  |

Else If reason is equal to `REQUIRES_AREA`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x0E | 4 / - | [Area](area.md) | area |  |  |

Else If reason is equal to `EQUIPPED_ITEM_CLASS`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x12 | 4 / Little | u32 | equipped_item_class |  |  |
| 0x16 | 4 / Little | u32 | equipped_item_subclass_mask |  |  |
| 0x1A | 4 / Little | u32 | equipped_item_inventory_type_mask |  |  |

