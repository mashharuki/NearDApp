/*
 * SmartContract for Nearprotocol
 */
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};

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
    // booking room data
    bookings_per_guest: HashMap<AccountId, HashMap<CheckInDate, RoomId>>,
}

// Booked Room Data Struct
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct BookedRoom {
    room_id: RoomId,
    name: String,
    check_in_date: CheckInDate,
    guest_id: AccountId,
    status: UsageStatus,
}

// Guest Booked Room data struct
#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuestBookedRoom {
    owner_id: AccountId,
    name: String,
    check_in_date: CheckInDate,
}

// Avaliable Room Data Struct
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AvailableRoom {
    room_id: RoomId,
    owner_id: AccountId,
    name: String,
    image: String,
    beds: u8,
    description: String,
    location: String,
    price: U128,
}

// Default trait
impl Default for Contract {
    // initial Contract
    fn default() -> Self {
        Self {
            rooms_per_owner: LookupMap::new(b"m"),
            rooms_by_id: HashMap::new(),
            bookings_per_guest: HashMap::new(),
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
     * book room function
     */
    #[payable]
    pub fn book_room(&mut self, room_id: RoomId, check_in_date: CheckInDate) {
        // get room data
        let room = self
            .rooms_by_id
            .get_mut(&room_id)
            .expect("ERR_NOT_FOUND_ROOM");
        // get accountID
        let account_id = env::signer_account_id();

        // sende near
        let deposit = env::attached_deposit();
        // chast
        let room_price: u128 = room.price.clone().into();
        // check
        assert_eq!(deposit, room_price, "ERR_DEPOSIT_IS_INCORRECT");

        // call insert methos
        room.booked_info.insert(check_in_date.clone(), account_id.clone());

        let owner_id = room.owner_id.clone();
        // call add_booking_to_guest function
        self.add_booking_to_guest(account_id, room_id, check_in_date);

        // pay near to owner
        Promise::new(owner_id).transfer(deposit);
    }

    /**
     * change status of room function
     * Stay -> Avaliable 
     */
    pub fn change_status_to_available(
        &mut self,
        room_id: RoomId,
        check_in_date: CheckInDate,
        guest_id: AccountId,
    ) {
        // get room data
        let mut room = self
            .rooms_by_id
            .get_mut(&room_id)
            .expect("ERR_NOT_FOUND_ROOM");

        // remove
        room.booked_info
            .remove(&check_in_date)
            .expect("ERR_NOT_FOUND_DATE");

         // change status to Available
        room.status = UsageStatus::Available;

        // remove booking room data
        self.remove_booking_from_guest(guest_id, check_in_date);
    }

    /**
     * change status of room function
     * Avaliable  -> Stay
     */
    pub fn change_status_to_stay(&mut self, room_id: RoomId, check_in_date: CheckInDate) {
        // get room data
        let mut room = self
            .rooms_by_id
            .get_mut(&room_id)
            .expect("ERR_NOT_FOUND_ROOM");

         // change status to Stay
        room.status = UsageStatus::Stay { check_in_date };
    }

    /**
     * check room status function
     */
    pub fn is_available(&self, room_id: RoomId) -> bool {
        // get room data
        let room = self.rooms_by_id.get(&room_id).expect("ERR_NOT_FOUND_ROOM");

        if room.status != UsageStatus::Available {
            return false;
        }
        
        return true;
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

    /**
     * get booking data function
     */
    pub fn get_booking_info_for_owner(&self, owner_id: AccountId) -> Vec<BookedRoom> {
        // create vector
        let mut booked_rooms = vec![];

        match self.rooms_per_owner.get(&owner_id) {
            Some(rooms) => {
                for room_id in rooms.iter() {
                    let room = self.rooms_by_id.get(room_id).expect("ERR_NOT_FOUND_ROOM");
                    
                    if room.booked_info.len() == 0 {
                        continue;
                    }
                    // create booking data
                    for (date, guest_id) in room.booked_info.clone() {
                        
                        let status: UsageStatus;

                        match &room.status {
                            UsageStatus::Available => status = UsageStatus::Available,
                            UsageStatus::Stay { check_in_date } => {
                                if date == check_in_date.clone() {
                                    status = UsageStatus::Stay {
                                        check_in_date: check_in_date.clone(),
                                    }
                                } else {
                                    status = UsageStatus::Available;
                                }
                            }
                        }
                        // create booked room data
                        let booked_room = BookedRoom {
                            room_id: room_id.to_string(),
                            name: room.name.clone(),
                            check_in_date: date,
                            guest_id,
                            status,
                        };
                        // push
                        booked_rooms.push(booked_room);
                    }
                }
                booked_rooms
            }
            // no data
            None => booked_rooms,
        }
    }

    /**
     * get booking info function
     */
    pub fn get_booking_info_for_guest(&self, guest_id: AccountId) -> Vec<GuestBookedRoom> {
        // create vector
        let mut guest_info: Vec<GuestBookedRoom> = vec![];

        match self.bookings_per_guest.get(&guest_id) {
            Some(save_booked_info) => {
                for (check_in_date, room_id) in save_booked_info {
                    let room = self.rooms_by_id.get(room_id).expect("ERR_NOT_FOUND_ROOM");
                    // create guest booked room data
                    let info = GuestBookedRoom {
                        owner_id: room.owner_id.clone(),
                        name: room.name.clone(),
                        check_in_date: check_in_date.clone(),
                    };
                    // push
                    guest_info.push(info);
                }
                guest_info
            }
            // no data
            None => guest_info,
        }
    }

    /**
     * get available rooms function
     */
    pub fn get_available_rooms(&self, check_in_date: CheckInDate) -> Vec<AvailableRoom> {
        // create vector
        let mut available_rooms = vec![];

        for (room_id, room) in self.rooms_by_id.iter() {
            match room.booked_info.get(&check_in_date) {
                
                Some(_) => continue,
                // if room_id can be booked
                None => {
                    // create Avalilable Room data
                    let available_room = AvailableRoom {
                        room_id: room_id.clone(),
                        owner_id: room.owner_id.clone(),
                        name: room.name.clone(),
                        beds: room.beds,
                        image: room.image.clone(),
                        description: room.description.clone(),
                        location: room.location.clone(),
                        price: room.price,
                    };
                    // push
                    available_rooms.push(available_room);
                }
            }
        }
        available_rooms
    }
}

// Private fuctions for Contract
impl Contract {
    
    /**
     * add booking to guest fuction
     */
    fn add_booking_to_guest(
        &mut self,
        guest_id: AccountId,
        room_id: RoomId,
        check_in_date: CheckInDate,
    ) {
        match self.bookings_per_guest.get_mut(&guest_id) {
            // if guest has already other booking data
            Some(booked_date) => {
                booked_date.insert(check_in_date.clone(), room_id);
            }
            // first time
            None => {
                let mut new_guest_date = HashMap::new();
                new_guest_date.insert(check_in_date.clone(), room_id);
                self.bookings_per_guest.insert(guest_id, new_guest_date);
            }
        }
    }

    /**
     * remove booking data function
     */
    fn remove_booking_from_guest(&mut self, guest_id: AccountId, check_in_date: CheckInDate) {
        // get booking data 
        let book_info = self
            .bookings_per_guest
            .get_mut(&guest_id)
            .expect("ERR_NOT_FOUND_GUEST");

        // remove
        book_info
            .remove(&check_in_date)
            .expect("ERR_NOT_FOUND_BOOKED");

        // if book infomation is empty, booking data is also removed
        if book_info.is_empty() {
            self.bookings_per_guest.remove(&guest_id);
        }
    }
}

/**
 * ========================================================
 * test code
 * ========================================================
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    /**
     * create virtual Blockchain env
     */
    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .signer_account_id(accounts(1))
            // 使用するメソッドをbooleanで指定(viewメソッドはtrue, changeメソッドはfalse)
            .is_view(is_view);
        builder
    }

    /**
     * test code for getting registered room data
     */
    #[test]
    fn add_then_get_registered_rooms() {
        // call get_context fuction
        let context = get_context(false);
        testing_env!(context.build());

        let mut contract = Contract::default();
        // add booking data
        contract.add_room_to_owner(
            "101".to_string(),
            "test.img".to_string(),
            1,
            "This is 101 room".to_string(),
            "Tokyo".to_string(),
            U128(10),
        );
        // add booking data
        contract.add_room_to_owner(
            "201".to_string(),
            "test.img".to_string(),
            1,
            "This is 201 room".to_string(),
            "Tokyo".to_string(),
            U128(10),
        );

        // get accountID
        let owner_id = env::signer_account_id();
        // get all booking data
        let all_rooms = contract.get_rooms_registered_by_owner(owner_id);
        // check
        assert_eq!(all_rooms.len(), 2);
    }

    /**
     * test code for getting no booking data
     */
    #[test]
    fn no_registered_room() {
        // call get_context fuction
        let context = get_context(true);
        testing_env!(context.build());
        let contract = Contract::default();
        // get booking data
        let no_registered_room = contract.get_rooms_registered_by_owner(accounts(0));
        // check
        assert_eq!(no_registered_room.len(), 0);
    }

    /**
     * test code for adding booking data
     */
    #[test]
    fn add_then_get_available_rooms() {
        // call get_context fuction
        let mut context = get_context(false);
        testing_env!(context.build());

        let mut contract = Contract::default();
        // add booking data
        contract.add_room_to_owner(
            "101".to_string(),
            "test.img".to_string(),
            1,
            "This is 101 room".to_string(),
            "Tokyo".to_string(),
            U128(10),
        );
        // add booking data
        contract.add_room_to_owner(
            "201".to_string(),
            "test.img".to_string(),
            1,
            "This is 201 room".to_string(),
            "Tokyo".to_string(),
            U128(10),
        );

        // `get_available_rooms`をコールするアカウントを設定
        testing_env!(context.signer_account_id(accounts(2)).build());
        let available_rooms = contract.get_available_rooms("2222-01-01".to_string());
        // check
        assert_eq!(available_rooms.len(), 2);
    }

    /**
     * test code for getting no available room data
     */
    #[test]
    fn no_available_room() {
        // call get_context fuction
        let context = get_context(true);
        testing_env!(context.build());
        let contract = Contract::default();
        // call get_available_rooms function
        let available_rooms = contract.get_available_rooms("2222-01-01".to_string());
        assert_eq!(available_rooms.len(), 0);
    }

    /**
     * test code for changing booking room status
     */
    #[test]
    fn book_room_then_change_status() {
        // call get_context fuction
        let mut context = get_context(false);

        // init balance
        context.account_balance(10);
        context.attached_deposit(10);

        testing_env!(context.build());
        // get signer info
        let owner_id = env::signer_account_id();
        let mut contract = Contract::default();

        // call for add_room_to_owner function
        contract.add_room_to_owner(
            "101".to_string(),
            "test.img".to_string(),
            1,
            "This is 101 room".to_string(),
            "Tokyo".to_string(),
            U128(10),
        );

        ///////////////////
        // CHECK BOOKING //
        ///////////////////
        // set accountID
        testing_env!(context.signer_account_id(accounts(2)).build());

        let check_in_date: String = "2222-01-01".to_string();
        // call get_available room data
        let available_rooms = contract.get_available_rooms(check_in_date.clone());

        // call book_room function
        contract.book_room(available_rooms[0].room_id.clone(), check_in_date.clone());

        // get booking room datas
        let booked_rooms = contract.get_booking_info_for_owner(owner_id.clone());
        // check
        assert_eq!(booked_rooms.len(), 1);
        assert_eq!(booked_rooms[0].check_in_date, check_in_date);
        assert_eq!(booked_rooms[0].guest_id, accounts(2));

        
        let guest_booked_rooms = contract.get_booking_info_for_guest(accounts(2));
        // check data for guest
        assert_eq!(guest_booked_rooms.len(), 1);
        assert_eq!(guest_booked_rooms[0].owner_id, owner_id);

        /////////////////////////
        // CHECK CHANGE STATUS //
        /////////////////////////
        // set accountID
        testing_env!(context.signer_account_id(accounts(1)).build());

        // check status
        let is_available = contract.is_available(booked_rooms[0].room_id.clone());
        // check
        assert_eq!(is_available, true);

        // change status（Available -> Stay）
        contract.change_status_to_stay(booked_rooms[0].room_id.clone(), check_in_date.clone());
        // get booking room infomations
        let booked_rooms = contract.get_booking_info_for_owner(owner_id.clone());
        // check
        assert_ne!(booked_rooms[0].status, UsageStatus::Available);

        // check status
        let is_available = contract.is_available(booked_rooms[0].room_id.clone());
        // check
        assert_eq!(is_available, false);

        // change status（Stay -> Available）
        contract.change_status_to_available(
            available_rooms[0].room_id.clone(),
            check_in_date.clone(),
            booked_rooms[0].guest_id.clone(),
        );

        // check
        let booked_rooms = contract.get_booking_info_for_owner(owner_id.clone());
        assert_eq!(booked_rooms.len(), 0);

        // check
        let guest_booked_info = contract.get_booking_info_for_guest(accounts(2));
        assert_eq!(guest_booked_info.len(), 0);
    }
}
