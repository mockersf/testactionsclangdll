use std::fs;
use std::sync::Arc;

#[derive(Debug)]
pub struct Ship {
    pub name: String,
    pub sprite: String,
}

#[derive(Debug)]
pub struct Player {
    pub ship: Arc<Ship>,
}

#[derive(Debug)]
pub struct Object {
    pub sprite: Option<String>,
    pub distance: f32,
    pub period: f32,
}

#[derive(Debug)]
pub struct System {
    pub name: String,
    pub objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Game {
    pub player: Player,
    pub systems: Vec<System>,
    pub ships: Vec<Arc<Ship>>,
}

pub fn start_from_es(path: &str) -> Game {
    let es_game_data_source =
        fs::read_to_string(path).expect("Something went wrong reading the file");

    start_from_es_data(&es_game_data_source)
}

pub fn start_from_es_data(es_game_data_source: &str) -> Game {
    let es_game_data = es_data_parser::parse(es_game_data_source);

    let ships = es_game_data
        .iter()
        .filter_map(|object| {
            if let es_data_parser::Object::Ship(ship) = object {
                Some(ship)
            } else {
                None
            }
        })
        .map(|ship| {
            Arc::new(Ship {
                name: String::from(ship.name),
                sprite: match ship.sprite {
                    es_data_parser::Sprite::Simple(sprite) => String::from(sprite),
                    es_data_parser::Sprite::Sprite { name, .. } => format!("{}=0", name),
                },
            })
        })
        .collect::<Vec<_>>();

    let systems = es_game_data
        .iter()
        .filter_map(|object| {
            if let es_data_parser::Object::System(system) = object {
                Some(system)
            } else {
                None
            }
        })
        .map(|system| System {
            name: String::from(system.name),
            objects: system
                .objects
                .iter()
                .map(|object| Object {
                    sprite: object.sprite.map(String::from),
                    distance: object.distance.unwrap_or(0.0),
                    period: object.period,
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    Game {
        player: Player {
            ship: ships
                .iter()
                .find(|ship| ship.name == "Shuttle")
                .unwrap()
                .clone(),
        },
        systems,
        ships,
    }
}
