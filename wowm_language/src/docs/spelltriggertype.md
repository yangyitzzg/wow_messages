# SpellTriggerType

## Client Version 1

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L31).

```rust,ignore
enum SpellTriggerType : u8 {
    ON_USE = 0;
    ON_EQUIP = 1;
    CHANCE_ON_HIT = 2;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `ON_USE` | 0 (0x00) |  |  |
| `ON_EQUIP` | 1 (0x01) |  |  |
| `CHANCE_ON_HIT` | 2 (0x02) |  |  |

Used in:
* [ItemSpells](itemspells.md)

## Client Version 2, Client Version 3

### Comment

azerothcore: `ItemSpelltriggerType` 5 might have changed on 2.4.3/3.0.3: Such auras will be applied on item pickup and removed on item loss - maybe on the other hand the item is destroyed if the aura is removed ('removed on death' of spell 57348 makes me think so)

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:54`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L54).

```rust,ignore
enum SpellTriggerType : u8 {
    ON_USE = 0;
    ON_EQUIP = 1;
    CHANCE_ON_HIT = 2;
    SERVER_SIDE_SCRIPT = 3;
    SOULSTONE = 4;
    NO_EQUIP_COOLDOWN = 5;
    LEARN_SPELL_ID = 6;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `ON_USE` | 0 (0x00) |  |  |
| `ON_EQUIP` | 1 (0x01) |  |  |
| `CHANCE_ON_HIT` | 2 (0x02) |  |  |
| `SERVER_SIDE_SCRIPT` | 3 (0x03) |  | cmangos-tbc: Only used by 23442, Glowing Sanctified Crystal which is used for a Hellfire Peninsula quest.<br/>Unknown why exactly it does not use the normal triggers. |
| `SOULSTONE` | 4 (0x04) |  |  |
| `NO_EQUIP_COOLDOWN` | 5 (0x05) |  |  |
| `LEARN_SPELL_ID` | 6 (0x06) |  |  |

Used in:
* [ItemSpells](itemspells.md)

