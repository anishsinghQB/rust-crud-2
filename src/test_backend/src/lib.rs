use candid::{CandidType, Decode, Encode};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

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

#[derive(CandidType, Serialize, Deserialize)]
enum Error {
    NotFound { msg: String },
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
    static  MEMORY_MANAGER : RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER : RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),0)
        .expect("Error while creating ID counter")
     );

    static STORAGE : RefCell<StableBTreeMap<u64,UserArg,Memory>> = RefCell::new(
    StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
}

fn get_msg(id: &u64) -> Option<UserArg> {
    STORAGE.with(|data| data.borrow().get(id))
}

#[query]
fn get_message(id: u64) -> Result<UserArg, Error> {
    match get_msg(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a  message with {} id is not found", id),
        }),
    }
}

fn add_post(user_data: &UserArg) {
    STORAGE.with(|data| data.borrow_mut().insert(user_data.id, user_data.clone()));
}

#[update]
fn upload_post(user_arg : ActualUserArg)-> Option<UserArg>{
    let _id = ID_COUNTER.with(|data|{
        let current_id: u64 = *data.borrow().get();
        data.borrow_mut().set(current_id)
    })
    .expect("id can not increse");

    let post_data : UserArg  = UserArg {
         id : _id,
         user_name: user_arg.user_name,
         post_title : user_arg.post_title,
         post_description : user_arg.post_description
    };
    add_post(&post_data);
    Some(post_data)
}

#[update]
fn update_post_details(payload : UserArg)->Result<UserArg, Error>{
    match STORAGE.with(|data| data.borrow().get(&payload.id)) {
        Some(mut message) =>{
            message.user_name = payload.user_name;
            message.post_description = payload.post_description;
            message.post_title = payload.post_title;
            add_post(&message);
            Ok(message)
        }
        None => Err(Error::NotFound { msg: format!("can not updated a post details with {}id", payload.id) })
    }
}

#[update]
fn delete_user_post(id : u64)->Result<UserArg, Error>{
    match STORAGE.with(|data|data.borrow_mut().remove(&id)) {
        Some(message)=> Ok(message),
        None => Err(Error::NotFound { msg: format!("can not delete post with {} id", id) })
    }
}

ic_cdk::export_candid!();
