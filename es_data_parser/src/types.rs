use derive_builder::Builder;

/// Mortgage owned by a player
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
#[builder(setter(into))]
pub struct Mortgage {
    /// amount of mortgage
    pub principal: u64,
    /// interest rate
    pub interest: f32,
    /// term by which mortgage is due
    pub term: u16,
}

/// Account of a player
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
#[builder(setter(into))]
pub struct Account {
    /// how much he currently has
    pub credits: u64,
    /// his credit score
    pub score: u32,
    /// his current mortgage
    pub mortgage: Mortgage,
}

/// A date
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
#[builder(setter(into))]
pub struct Date {
    /// the year
    pub year: u16,
    /// the month
    pub month: u8,
    /// the day
    pub day: u8,
}

/// A fleet
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Fleet<'a> {
    /// kind of the fleet
    pub kind: &'a str,
    /// count of ships in the fleet
    pub count: u16,
}

/// Tribute given by a planet
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Tribute<'a> {
    /// amount given for tribute
    pub value: u32,
    /// menace threshold at which planet will reply for tribute
    pub threshold: u32,
    /// fleet that will protect planet
    pub fleet: Fleet<'a>,
}

/// Start point for the player
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Start<'a> {
    /// start date
    pub date: Date,
    /// start system
    pub system: &'a str,
    /// start planet
    pub planet: &'a str,
    /// start account
    pub account: Account,
    /// start set
    pub set: &'a str,
}

/// A planet
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Planet<'a> {
    /// name of the planet
    pub name: &'a str,
    /// attributes of the planet
    #[builder(default)]
    pub attributes: Vec<&'a str>,
    /// landscape to display for the planet
    #[builder(default)]
    pub landscape: Option<&'a str>,
    /// government of the planet, if different from the parent system
    #[builder(default)]
    pub government: Option<&'a str>,
    /// music to play on landing
    #[builder(default)]
    pub music: Option<&'a str>,
    /// description of the planet, each &str is a line
    pub description: Vec<&'a str>,
    /// description of the spaceport, each &str is a line
    #[builder(default)]
    pub spaceport: Vec<&'a str>,
    /// shipyard, each &str is a set of ships sold
    #[builder(default)]
    pub shipyard: Vec<&'a str>,
    /// outfitter, each &str is a set of outfits sold
    #[builder(default)]
    pub outfitter: Vec<&'a str>,
    /// factor for bribe (?)
    #[builder(default)]
    pub bribe: Option<f32>,
    /// security of the planet (?)
    #[builder(default)]
    pub security: Option<f32>,
    /// tribute for this planet
    #[builder(default)]
    pub tribute: Option<Tribute<'a>>,
    /// required reputation with planet faction to land
    #[builder(default)]
    pub required_reputation: Option<f32>,
}

/// A position
#[derive(Debug, Clone, Copy, PartialEq, Builder)]
#[builder(setter(into))]
pub struct Position {
    /// x pos
    pub x: f64,
    /// y pos
    pub y: f64,
}

/// A galaxy
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Galaxy<'a> {
    /// it's position
    pub pos: Position,
    /// it's name
    pub name: &'a str,
    /// it's sprite
    pub sprite: Option<&'a str>,
}

/// An asteroid
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Asteroids<'a> {
    /// it's name
    pub name: &'a str,
    /// (?)
    pub first_value: u32,
    /// (?)
    pub second_value: f32,
}

/// A minable
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Minables<'a> {
    /// it's name
    pub name: &'a str,
    /// (?)
    pub first_value: u32,
    /// (?)
    pub second_value: f32,
}

/// A trade good with a price
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Trade<'a> {
    /// it's name
    pub name: &'a str,
    /// price
    pub price: u32,
}

/// An object in a system
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct SystemObject<'a> {
    /// it's name
    pub name: Option<&'a str>,
    /// it's sprite
    pub sprite: Option<&'a str>,
    /// distance
    pub distance: Option<f32>,
    /// period
    pub period: f32,
    /// offset
    pub offset: Option<f32>,
    /// related objects
    pub objects: Vec<SystemObject<'a>>,
}

/// A system
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct System<'a> {
    /// it's name
    pub name: &'a str,
    /// it's position
    pub pos: Position,
    /// it's government
    pub government: &'a str,
    /// habitable (?)
    pub habitable: f32,
    /// belt (?)
    pub belt: Option<u32>,
    /// haze type
    pub haze: Option<&'a str>,
    /// links to other systems
    pub links: Vec<&'a str>,
    /// asteroids present in the system
    pub asteroids: Vec<Asteroids<'a>>,
    /// minables present in the system
    pub minables: Vec<Minables<'a>>,
    /// trade goods that are sold here
    pub trades: Vec<Trade<'a>>,
    /// fleets present in the system
    pub fleets: Vec<Fleet<'a>>,
    /// objects present in the system
    pub objects: Vec<SystemObject<'a>>,
}

/// weapon of a ship (?)
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
pub struct ShipWeapon {
    /// it's blast radius
    pub blast_radius: u32,
    /// it's shield damage
    pub shield_damage: u32,
    /// it's hull damage
    pub hull_damage: u32,
    /// it's hit force
    pub hit_force: u32,
}

/// Attributes of a ship
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct ShipAttributes<'a> {
    /// licences needed to pilot this ship
    #[builder(default)]
    pub licenses: Vec<&'a str>,
    /// it's category
    pub category: &'a str,
    /// it's cost
    pub cost: u32,
    /// it's shield
    #[builder(default)]
    pub shields: u32,
    /// it's hull strength
    pub hull: u32,
    /// is it an automaton
    #[builder(default)]
    pub automaton: bool,
    /// it's required crew count
    #[builder(default)]
    pub required_crew: u32,
    /// it's bunk count
    #[builder(default)]
    pub bunks: u32,
    /// it's mass
    pub mass: u32,
    /// it's drag
    pub drag: f32,
    /// it's heat dissipation
    pub heat_dissipation: f32,
    /// it's fuel capacity
    #[builder(default)]
    pub fuel_capacity: u32,
    /// it's cargo space
    #[builder(default)]
    pub cargo_space: u32,
    /// it's outfit space
    pub outfit_space: u32,
    /// it's weapon capacity
    #[builder(default)]
    pub weapon_capacity: u32,
    /// it's engine capacity
    pub engine_capacity: u32,
    /// it's weapon (?)
    pub weapon: ShipWeapon,
}

/// a sprite
#[derive(Debug, PartialEq, Clone)]
pub enum Sprite<'a> {
    /// Complex sprite with multiple frames
    Sprite {
        /// name of the sprite
        name: &'a str,
        /// (?)
        frame_time: u32,
        /// (?)
        delay: u32,
        /// (?)
        random_start_frame: bool,
    },
    /// Simple sprite
    Simple(&'a str),
}

/// A ship
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct Ship<'a> {
    /// name of the ship
    pub name: &'a str,
    /// subclass of the ship
    pub subclass: Option<&'a str>,
    /// plural form of the name
    #[builder(setter(into), default)]
    pub plural: Option<&'a str>,
    /// sprite of the ship
    pub sprite: Sprite<'a>,
    /// thumbnail of the ship
    pub thumbnail: &'a str,
    /// attributes of the ship
    pub attributes: ShipAttributes<'a>,
    /// outfits of the ship
    pub outfits: Vec<(&'a str, u32)>,
    /// engine locations and (?)
    pub engine: Vec<(f32, f32, Option<f32>)>,
    /// gun mount locations and what they hold
    #[builder(default)]
    pub gun: Vec<(f32, f32, Option<&'a str>)>,
    /// turret mount locations and what they hold
    #[builder(default)]
    pub turret: Vec<(f32, f32, Option<&'a str>)>,
    /// fighter mount locations and wherethey are
    #[builder(default)]
    pub fighter: Vec<(f32, f32, Option<&'a str>)>,
    /// drone mount locations and where they are
    #[builder(default)]
    pub drone: Vec<(f32, f32, Option<&'a str>)>,
    /// leaks (?)
    #[builder(default)]
    pub leak: Vec<(&'a str, u32, u32)>,
    /// explosion on death and tiling (?)
    pub explode: Vec<(&'a str, u32)>,
    /// final explosion
    #[builder(setter(into), default)]
    pub final_explode: Option<&'a str>,
    /// description
    pub description: Vec<&'a str>,
}

/// list of top level objects that can be parsed
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Object<'a> {
    /// player start
    Start(Start<'a>),
    /// a planet
    Planet(Planet<'a>),
    /// a galaxy
    Galaxy(Galaxy<'a>),
    /// a system
    System(System<'a>),
    /// a ship
    Ship(Ship<'a>),
}
