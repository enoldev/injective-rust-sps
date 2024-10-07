mod pb;

use pb::sf::substreams::cosmos::v1::EventList;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;


#[substreams::handlers::map]
pub fn graph_out(event_list: EventList) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    let mut i = 0;
    for event in event_list.events {
        let id = std::format!("{}-{}", event.transaction_hash, i.to_string());

        if let Some(inner_event) = event.event {
            tables
                .create_row("Event", id)
                .set("type", inner_event.r#type);
        }

        i += 1;
    }

    Ok(tables.to_entity_changes())
}
