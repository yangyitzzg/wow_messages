# PetNameInvalidReason

## Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm#L6).

```rust,ignore
enum PetNameInvalidReason : u8 {
    INVALID = 1;
    NO_NAME = 2;
    TOO_SHORT = 3;
    TOO_LONG = 4;
    MIXED_LANGUAGES = 6;
    PROFANE = 7;
    RESERVED = 8;
    THREE_CONSECUTIVE = 11;
    INVALID_SPACE = 12;
    CONSECUTIVE_SPACES = 13;
    RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS = 14;
    RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END = 15;
    DECLENSION_DOESNT_MATCH_BASE_NAME = 16;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `INVALID` | 1 (0x01) |  |  |
| `NO_NAME` | 2 (0x02) |  |  |
| `TOO_SHORT` | 3 (0x03) |  |  |
| `TOO_LONG` | 4 (0x04) |  |  |
| `MIXED_LANGUAGES` | 6 (0x06) |  |  |
| `PROFANE` | 7 (0x07) |  |  |
| `RESERVED` | 8 (0x08) |  |  |
| `THREE_CONSECUTIVE` | 11 (0x0B) |  |  |
| `INVALID_SPACE` | 12 (0x0C) |  |  |
| `CONSECUTIVE_SPACES` | 13 (0x0D) |  |  |
| `RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS` | 14 (0x0E) |  |  |
| `RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END` | 15 (0x0F) |  |  |
| `DECLENSION_DOESNT_MATCH_BASE_NAME` | 16 (0x10) |  |  |

Used in:
* [SMSG_PET_NAME_INVALID](smsg_pet_name_invalid.md)

