# EmoteFlags

## Client Version 1.12

### Comment

Used in `Emotes.dbc`.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/external/emote_flags.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/emote_flags.wowm#L1).

```rust,ignore
flag EmoteFlags : u8 {
    TALK = 0x08;
    QUESTION = 0x10;
    EXCLAMATION = 0x20;
    SHOUT = 0x40;
    LAUGH = 0x80;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `TALK` | 8 (0x08) |  |  |
| `QUESTION` | 16 (0x10) |  |  |
| `EXCLAMATION` | 32 (0x20) |  |  |
| `SHOUT` | 64 (0x40) |  |  |
| `LAUGH` | 128 (0x80) |  |  |

Used in:
