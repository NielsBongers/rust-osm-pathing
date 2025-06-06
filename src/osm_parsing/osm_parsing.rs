use log::{error, warn};

use crate::data_handling::NodeSubset;
use crate::osm_parsing::{CurrentlyReading, StateMachine};
use crate::osm_parsing::{Node, Way};
use crate::utils::attributes::read_attributes;

use quick_xml::events::Event;
use quick_xml::Reader;

use std::collections::HashMap;
use std::path::Path;

pub fn parse_xml(file_path: &Path) -> (HashMap<u64, Node>, HashMap<u64, Way>, Vec<NodeSubset>) {
    let mut reader = Reader::from_file(file_path).expect("Failed to create reader from file");

    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    let mut node_map: HashMap<u64, Node> = HashMap::new();
    let mut way_map: HashMap<u64, Way> = HashMap::new();
    let node_subset: Vec<NodeSubset> = Vec::new();

    let mut state_machine = StateMachine::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                error!("Error at position {}: {:?}", reader.error_position(), e);
                return (node_map, way_map, node_subset);
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let contents = read_attributes(e.attributes());
                match e.name().as_ref() {
                    b"osm" => (),
                    b"node" => {
                        let node = Node::new(contents);

                        state_machine.update(CurrentlyReading::Node(node.id));
                        node_map.insert(node.id, node);
                    }
                    b"way" => {
                        let way = Way::new(contents);

                        state_machine.update(CurrentlyReading::Way(way.id));
                        way_map.insert(way.id, way);
                    }
                    b"relation" => {
                        state_machine.update(CurrentlyReading::Relation(0));
                    }
                    _ => (),
                };
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                _ => state_machine.update(CurrentlyReading::None),
            },
            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"node" => {
                        let contents: HashMap<String, String> = read_attributes(e.attributes());
                        let node: Node = Node::new(contents);

                        state_machine.update(CurrentlyReading::Node(node.id));
                        node_map.insert(node.id, node);
                    }
                    b"tag" => {
                        let contents: HashMap<String, String> = read_attributes(e.attributes());

                        assert!(
                            contents.len() == 2,
                            "Tag without two entries: {:?}",
                            contents
                        );

                        let key = contents.get("k").unwrap().to_string();
                        let value = contents.get("v").unwrap().to_string();
                        match state_machine.current_status() {
                            CurrentlyReading::Node(id) => {
                                if let Some(node) = node_map.get_mut(&id) {
                                    // info!("Encountered a tag: {}: {}", key, value);
                                    node.tags.insert(key, value);
                                }
                            }
                            CurrentlyReading::Way(id) => {
                                if let Some(way) = way_map.get_mut(&id) {
                                    way.tags.insert(key, value);
                                }
                            }
                            CurrentlyReading::Relation(_) => (),
                            _ => warn!(
                                "Trying to read tag: encountered state machine in {:?}",
                                state_machine.current_status()
                            ),
                        }
                    }
                    b"nd" => {
                        let contents = read_attributes(e.attributes());
                        assert!(
                            contents.len() == 1,
                            "Multiple key/value pairs in a way's node: {:?}",
                            contents
                        );

                        for (_, value) in contents.iter() {
                            let node_id = value.parse::<u64>().expect("Failed to parse id");

                            match state_machine.current_status() {
                                CurrentlyReading::Node(_) => {
                                    warn!("There should NOT be a node ID here!")
                                }
                                CurrentlyReading::Way(id) => {
                                    if let Some(way) = way_map.get_mut(&id) {
                                        way.node_ids.push(node_id);
                                    }
                                }
                                _ => warn!(
                                    "Trying to read tag: encountered state machine in {:?}",
                                    state_machine.current_status()
                                ),
                            }
                        }
                    }
                    b"member" => (),
                    b"bounds" => (),
                    b"meta" => (),
                    _ => warn!("Encountered name {:?}", e.name()),
                };
            }
            _ => (),
        };
    }
    buf.clear();

    (node_map, way_map, node_subset)
}
