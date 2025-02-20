# ItemSpells

## Client Version 1

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L40).
```rust,ignore
struct ItemSpells {
    u32 spell;
    (u32)SpellTriggerType spell_trigger;
    i32 spell_charges;
    i32 spell_cooldown;
    u32 spell_category;
    i32 spell_category_cooldown;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | spell |  |  |
| 0x04 | 4 / - | [SpellTriggerType](spelltriggertype.md) | spell_trigger |  |  |
| 0x08 | 4 / Little | i32 | spell_charges |  | let the database control the sign here. negative means that the item should be consumed once the charges are consumed. |
| 0x0C | 4 / Little | i32 | spell_cooldown |  |  |
| 0x10 | 4 / Little | u32 | spell_category |  |  |
| 0x14 | 4 / Little | i32 | spell_category_cooldown |  |  |

## Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:71`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L71).
```rust,ignore
struct ItemSpells {
    u32 spell;
    (u32)SpellTriggerType spell_trigger;
    i32 spell_charges;
    i32 spell_cooldown;
    u32 spell_category;
    i32 spell_category_cooldown;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | spell |  |  |
| 0x04 | 4 / - | [SpellTriggerType](spelltriggertype.md) | spell_trigger |  |  |
| 0x08 | 4 / Little | i32 | spell_charges |  | let the database control the sign here. negative means that the item should be consumed once the charges are consumed. |
| 0x0C | 4 / Little | i32 | spell_cooldown |  |  |
| 0x10 | 4 / Little | u32 | spell_category |  |  |
| 0x14 | 4 / Little | i32 | spell_category_cooldown |  |  |

