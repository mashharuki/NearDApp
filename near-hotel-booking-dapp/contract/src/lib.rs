/*
 * SmartContract for Nearprotocol
 */
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

use std::collections::HashMap;

// Rooom ID
type RoomId = String;
// Data of CheckIn
type CheckInDate = String;

// UsageStatus
#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum UsageStatus {
    Available,                           
    Stay { check_in_date: CheckInDate }, 
}

// Struct RegisterRoom
#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisteredRoom {
    name: String,
    image: String,
    beds: u8,
    description: String,
    location: String,
    price: U128,
    status: UsageStatus,
}

// Struct Room data 
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Room {
    name: String,        // 部屋の名前
    owner_id: AccountId, // オーナーのアカウントID
    image: String,       // 部屋の画像（URL）
    beds: u8,            // ベッドの数
    description: String, // 部屋の説明
    location: String,    // 施設の場所
    price: U128,         // 一泊の宿泊料
    status: UsageStatus, // 利用状況
    booked_info: HashMap<CheckInDate, AccountId>, // 予約データ[宿泊日, 宿泊者のアカウントID]
}

// struct of Contract
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    rooms_per_owner: LookupMap<AccountId, Vec<RoomId>>,
    rooms_by_id: HashMap<RoomId, Room>,
}

// Default trait
impl Default for Contract {
    // initial Contract
    fn default() -> Self {
        Self {
            rooms_per_owner: LookupMap::new(b"m"),
            rooms_by_id: HashMap::new(),
        }
    }
}

// Contract Type
#[near_bindgen]
impl Contract {

    /**
     * add room function
     */
    pub fn add_room_to_owner(
        &mut self,
        name: String,
        image: String,
        beds: u8,
        description: String,
        location: String,
        price: U128,
    ) {
        // get account ID
        let owner_id = env::signer_account_id();
        // room ID
        let room_id = format!("{}{}", owner_id, name);

        // create Room data
        let new_room = Room {
            owner_id: owner_id.clone(),
            name,
            image,
            beds,
            description,
            location,
            price,
            status: UsageStatus::Available,
            booked_info: HashMap::new(),
        };

        // insert room data
        self.rooms_by_id.insert(room_id.clone(), new_room);

        // オーナーのアカウントIDと`RoomId`のVectorを紐付けて保存
        match self.rooms_per_owner.get(&owner_id) {
            // room data is already Registed 
            Some(mut rooms) => {
                rooms.push(room_id.clone());
                self.rooms_per_owner.insert(&owner_id, &rooms);
            }
            // first time
            None => {
                // create vector
                let new_rooms = vec![room_id];
                // insert room data
                self.rooms_per_owner.insert(&owner_id, &new_rooms);
            }
        }
    }

    /**
     * check room name is already existed 
     */
    pub fn exists(&self, owner_id: AccountId, room_name: String) -> bool {
        let room_id = format!("{}{}", owner_id, room_name);
        // check
        self.rooms_by_id.contains_key(&room_id)
    }

    /**
     * get rooms data function 
     */
    pub fn get_rooms_registered_by_owner(&self, owner_id: AccountId) -> Vec<RegisteredRoom> {
        // create vector
        let mut registered_rooms = vec![];

        match self.rooms_per_owner.get(&owner_id) {
            // if room data is already Registed 
            Some(rooms) => {
                // roop
                for room_id in rooms {
                    // get room data
                    let room = self.rooms_by_id.get(&room_id).expect("ERR_NOT_FOUND_ROOM");

                    let room_status: UsageStatus;
                    
                    match &room.status {
                        // if usage state is Avalilable
                        UsageStatus::Available => room_status = UsageStatus::Available,
                        // if usage state is Stay
                        UsageStatus::Stay { check_in_date } => {
                            room_status = UsageStatus::Stay {
                                check_in_date: check_in_date.clone(),
                            }
                        }
                    }

                    // create RegisteredRoom data
                    let resigtered_room = RegisteredRoom {
                        name: room.name.clone(),
                        beds: room.beds,
                        image: room.image.clone(),
                        description: room.description.clone(),
                        location: room.location.clone(),
                        price: room.price,
                        status: room_status,
                    };
                    // Vectorに追加
                    registered_rooms.push(resigtered_room);
                }
                // return rooms data
                registered_rooms
            }
            // no data
            None => registered_rooms,
        }
    }
}
