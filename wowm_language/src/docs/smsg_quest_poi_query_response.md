# SMSG_QUEST_POI_QUERY_RESPONSE

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L30).
```rust,ignore
smsg SMSG_QUEST_POI_QUERY_RESPONSE = 0x01E4 {
    u32 amount_of_quests;
    QuestPoiList[amount_of_quests] quests;
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
| 0x04 | 4 / Little | u32 | amount_of_quests |  |  |
| 0x08 | ? / - | [QuestPoiList](questpoilist.md)[amount_of_quests] | quests |  |  |

