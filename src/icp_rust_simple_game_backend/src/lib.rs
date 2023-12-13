#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define types for memory and ID management
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the Player struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Player {
    id: u64,
    username: String,
    level: u32,
    experience: u64,
    created_at: u64,
    achievements:Vec<u64>
}

// Implement Storable and BoundedStorable for Player
impl Storable for Player {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Player {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define the Achievement struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Achievement {
    id: u64,
    name: String,
    description: String,
    points: u32,
}

// Implement Storable and BoundedStorable for Achievement
impl Storable for Achievement {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Achievement {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define the InventoryItem struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InventoryItem {
    id: u64,
    name: String,
    description: String,
    quantity: u32,
    created_at: u64,
}

// Implement Storable and BoundedStorable for InventoryItem
impl Storable for InventoryItem {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InventoryItem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local storage for memory manager, ID counter, player storage, achievements, and inventory
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PLAYERS: RefCell<StableBTreeMap<u64, Player, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static ACHIEVEMENTS: RefCell<StableBTreeMap<u64, Achievement, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static INVENTORY: RefCell<StableBTreeMap<u64, InventoryItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

// Payloads for creating entities
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct AchievementPayload {
    name: String,
    description: String,
    points: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct InventoryItemPayload {
    name: String,
    description: String,
    quantity: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct PlayerPayload {
    username: String,
}

// Query function to get all players
#[ic_cdk::query]
fn get_all_players() -> Result<Vec<Player>, Error> {
    let players_map: Vec<(u64, Player)> = PLAYERS.with(|service| service.borrow().iter().collect());
    let players: Vec<Player> = players_map.into_iter().map(|(_, player)| player).collect();

    if !players.is_empty() {
        Ok(players)
    } else {
        Err(Error::NotFound {
            msg: "No players found.".to_string(),
        })
    }
}

// Query function to get a player by ID
#[ic_cdk::query]
fn get_player(id: u64) -> Result<Player, Error> {
    match _get_player(&id) {
        Some(player) => Ok(player),
        None => Err(Error::NotFound {
            msg: format!("Player with id={} not found.", id),
        }),
    }
}

// Internal function to get a player by ID
fn _get_player(id: &u64) -> Option<Player> {
    PLAYERS.with(|s| s.borrow().get(id))
}

// Update function to create a new player
#[ic_cdk::update]
fn create_player(payload: PlayerPayload) -> Option<Player> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let player = Player {
        id,
        username: payload.username,
        level: 1,
        experience: 0,
        created_at: time(),
        achievements: Vec::new()
    };
    do_insert_player(&player);
    Some(player)
}

// Internal function to insert a player into storage
fn do_insert_player(player: &Player) {
    PLAYERS.with(|service| service.borrow_mut().insert(player.id, player.clone()));
}

// Update function to update a player's level
#[ic_cdk::update]
fn update_player_level(id: u64, new_level: u32) -> Result<Player, Error> {
    let player_option: Option<Player> = _get_player(&id);

    match player_option {
        Some(mut player) => {
            player.level = new_level;
            do_insert_player(&player);
            Ok(player)
        }
        None => Err(Error::NotFound {
            msg: format!("Player with id={} not found.", id),
        }),
    }
}

// Query function to get all achievements
#[ic_cdk::query]
fn get_all_achievements() -> Result<Vec<Achievement>, Error> {
    let achievements_map: Vec<(u64, Achievement)> =
        ACHIEVEMENTS.with(|service| service.borrow().iter().collect());
    let achievements: Vec<Achievement> = achievements_map.into_iter().map(|(_, achievement)| achievement).collect();

    if !achievements.is_empty() {
        Ok(achievements)
    } else {
        Err(Error::NotFound {
            msg: "No achievements found.".to_string(),
        })
    }
}

// Query function to get an achievement by ID
#[ic_cdk::query]
fn get_achievement(id: u64) -> Result<Achievement, Error> {
    match _get_achievement(&id) {
        Some(achievement) => Ok(achievement),
        None => Err(Error::NotFound {
            msg: format!("Achievement with id={} not found.", id),
        }),
    }
}

// Internal function to get an achievement by ID
fn _get_achievement(id: &u64) -> Option<Achievement> {
    ACHIEVEMENTS.with(|s| s.borrow().get(id))
}

// Update function to create a new achievement
#[ic_cdk::update]
fn create_achievement(payload: AchievementPayload) -> Option<Achievement> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let achievement = Achievement {
        id,
        name: payload.name,
        description: payload.description,
        points: payload.points,
    };
    do_insert_achievement(&achievement);
    Some(achievement)
}

// Internal function to insert an achievement into storage
fn do_insert_achievement(achievement: &Achievement) {
    ACHIEVEMENTS.with(|service| service.borrow_mut().insert(achievement.id, achievement.clone()));
}

//update function to check conditions and award an achievment to a player
#[ic_cdk::update]
fn earn_achievement(player_id: u64, achievement_id: u64) -> Result<Player,Error>{
    let player_option: Option<Player> = _get_player(&player_id);
    let achievement_option: Option<Achievement> = _get_achievement(&achievement_id);

    match (player_option, achievement_option) {
        (Some(mut player), Some(achievement))=>{
            //check if the player already has the achievement
            if player.achievements.contains(&achievement_id){
                return Ok(player);
            }

            //check if player has enough points to earn achievement
            if player.experience >= achievement.points as u64 {
                //award the achievement to the player
                player.achievements.push(achievement_id);
                do_insert_player(&player);
                Ok(player)
            }else{
                Err(Error::InsufficientExperience{
                    msg: "Player does not have enough experience to earn the achievement".to_string(),
                })
            }
        }
    (None, _) => Err(Error::NotFound {
        msg: format!("Player with id={} not found",player_id),
    }),
    (_, None) => Err(Error::NotFound {
        msg: format!("Achievement with id={} not found",achievement_id),
    }),
    }
}

// Query function to get all inventory items
#[ic_cdk::query]
fn get_all_inventory_items() -> Result<Vec<InventoryItem>, Error> {
    let items_map: Vec<(u64, InventoryItem)> =
        INVENTORY.with(|service| service.borrow().iter().collect());
    let items: Vec<InventoryItem> = items_map.into_iter().map(|(_, item)| item).collect();

    if !items.is_empty() {
        Ok(items)
    } else {
        Err(Error::NotFound {
            msg: "No inventory items found.".to_string(),
        })
    }
}

// Query function to get an inventory item by ID
#[ic_cdk::query]
fn get_inventory_item(id: u64) -> Result<InventoryItem, Error> {
    match _get_inventory_item(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("Inventory item with id={} not found.", id),
        }),
    }
}

// Internal function to get an inventory item by ID
fn _get_inventory_item(id: &u64) -> Option<InventoryItem> {
    INVENTORY.with(|s| s.borrow().get(id))
}

// Update function to create a new inventory item
#[ic_cdk::update]
fn create_inventory_item(payload: InventoryItemPayload) -> Option<InventoryItem> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let item = InventoryItem {
        id,
        name: payload.name,
        description: payload.description,
        quantity: payload.quantity,
        created_at: time(),
    };
    do_insert_inventory_item(&item);
    Some(item)
}

// Internal function to insert an inventory item into storage
fn do_insert_inventory_item(item: &InventoryItem) {
    INVENTORY.with(|service| service.borrow_mut().insert(item.id, item.clone()));
}

// Update function to simulate using an inventory item
#[ic_cdk::update]
fn use_inventory_item(player_id: u64, item_id: u64) -> Result<Player, Error> {
    let player_option: Option<Player> = _get_player(&player_id);
    let item_option: Option<InventoryItem> = _get_inventory_item(&item_id);

    match (player_option, item_option) {
        (Some(mut player), Some(item)) => {
            if player.experience + item.quantity as u64 > u64::MAX {
                Err(Error::InvalidOperation {
                    msg: "Player experience overflow.".to_string(),
                })
            } else {
                player.experience += item.quantity as u64;
                do_insert_player(&player);
                // Remove the used item from the shared inventory
                INVENTORY.with(|service| service.borrow_mut().remove(&item_id));
                Ok(player)
            }
        }
        (None, _) => Err(Error::NotFound {
            msg: format!("Player with id={} not found.", player_id),
        }),
        (_, None) => Err(Error::NotFound {
            msg: format!("Inventory item with id={} not found.", item_id),
        }),
    }
}

// Define the Error enum
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidOperation { msg: String },
    InsufficientExperience {msg: String},

}

// Export Candid for the defined types and functions
ic_cdk::export_candid!();
