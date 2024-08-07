use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};
use candid::{CandidType, Decode, Encode};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;
pub type AdminIdCell = Cell<u64, Memory>;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct UserArg {
    pub id: u64,
    pub user_name: String,
    pub post_title: String,
    pub post_description: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ActualUserArg {
    pub user_name: String,
    pub post_title: String,
    pub post_description: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct AdminArg {
    pub admin_id: u64,
    pub admin_name: String,
    pub admin_access: String,
}


#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct AdminActuallArg {
    pub admin_name: String,
    pub admin_access: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum Error {
    NotFound { msg: String },
}


impl Storable for AdminArg {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}


impl Storable for UserArg {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

// impl BoundedStorable for UserArg {
//     const MAX_SIZE: u32 = 1024;
//     const IS_FIXED_SIZE: bool = false;
// }

thread_local! {
    pub static  MEMORY_MANAGER : RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static ID_COUNTER : RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),0)
        .expect("Error while creating ID counter")
     );
    pub static ADMIN_ID : RefCell<AdminIdCell> = RefCell::new(
        AdminIdCell::init(MEMORY_MANAGER.with(|data| data.borrow().get(MemoryId::new(0))),101)
        .expect("error while set the admin id")
    );

    pub static STORAGE : RefCell<StableBTreeMap<u64,UserArg,Memory>> = RefCell::new(
    StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    pub static ADMIN_STORAGE : RefCell<StableBTreeMap<u64, AdminArg, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(101))))
    );
}

pub fn get_msg(id: &u64) -> Option<UserArg> {
    STORAGE.with(|data| data.borrow().get(id))
}

pub fn get_admin_data(id: &u64)-> Option<AdminArg>{
    ADMIN_STORAGE.with(|admin_data| admin_data.borrow().get(&id))
}