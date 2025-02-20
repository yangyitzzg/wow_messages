# DefaultChannelFlags

## Client Version 1.12

### Comment

Used in `ChatChannels.dbc`.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/external/channel_flags.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/channel_flags.wowm#L1).

```rust,ignore
flag DefaultChannelFlags : u32 {
    NONE = 0x00;
    INITIAL = 0x01;
    ZONE_DEPENDENCY = 0x02;
    GLOBAL = 0x04;
    TRADE = 0x08;
    CITY_ONLY = 0x10;
    CITY_ONLY_2 = 0x20;
    DEFENCE = 0x10000;
    UNSELECTED = 0x40000;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `INITIAL` | 1 (0x01) |  |  |
| `ZONE_DEPENDENCY` | 2 (0x02) |  |  |
| `GLOBAL` | 4 (0x04) |  |  |
| `TRADE` | 8 (0x08) |  |  |
| `CITY_ONLY` | 16 (0x10) |  |  |
| `CITY_ONLY_2` | 32 (0x20) |  |  |
| `DEFENCE` | 65536 (0x10000) |  |  |
| `UNSELECTED` | 262144 (0x40000) |  |  |

Used in:
