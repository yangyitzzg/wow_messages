# QuestPoiList

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L1).
```rust,ignore
struct QuestPoiList {
    u32 quest_id;
    u32 amount_of_pois;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | quest_id |  |  |
| 0x04 | 4 / Little | u32 | amount_of_pois |  |  |

