use ic_cdk::{query, update};
use stable_impl::{get_msg,get_admin_data,UserArg,Error,AdminArg,AdminActuallArg, ActualUserArg,add_post,ID_COUNTER,ADMIN_ID,STORAGE, add_admin_data,ADMIN_STORAGE,is_anonymous};
mod stable_impl;

#[query(guard = "is_anonymous")]
fn get_message(id: u64) -> Result<UserArg, Error> {
    match get_msg(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a  message with {} id is not found", id),
        }),
    }
}
#[query]
fn get_admin_arg(id: u64)->Result<AdminArg, Error>{
    match get_admin_data(&id) {
        Some(admin_full_data) => Ok(admin_full_data),
        None => Err(Error::NotFound { msg: format!("admin data not found with {} id",id) }),
    }
}

#[update(guard = "is_anonymous")]
fn post_admin_data(admin_arg: AdminActuallArg) -> Option<AdminArg> {
    let admins_id = ADMIN_ID.with(|admin_data| {
        let admin_id = *admin_data.borrow().get();
        admin_data.borrow_mut().set(admin_id+1)
    })
    .expect("can not set the id");

    let admin_data = AdminArg{
        admin_id : admins_id,
       admin_name: admin_arg.admin_name,
       admin_access: admin_arg.admin_access,
    };
    add_admin_data(&admin_data);
    Some(admin_data)
}

#[update]
fn upload_post(user_arg: ActualUserArg) -> Option<UserArg> {
    let _id = ID_COUNTER
        .with(|data| {
            let current_id: u64 = *data.borrow().get();
            data.borrow_mut().set(current_id+1)
        })
        .expect("id can not increse");

    let post_data: UserArg = UserArg {
        id: _id,
        user_name: user_arg.user_name,
        post_title: user_arg.post_title,
        post_description: user_arg.post_description,
    };
    add_post(&post_data);
    Some(post_data)
}

#[update]
fn update_post_details(payload: UserArg) -> Result<UserArg, Error> {
    match STORAGE.with(|data| data.borrow().get(&payload.id)) {
        Some(mut message) => {
            message.user_name = payload.user_name;
            message.post_description = payload.post_description;
            message.post_title = payload.post_title;
            add_post(&message);
            Ok(message)
        }
        None => Err(Error::NotFound {
            msg: format!("can not updated a post details with {}id", payload.id),
        }),
    }
}

#[update]
fn update_admin_post_details(payload: AdminArg)-> Result<AdminArg, Error>{
    match ADMIN_STORAGE.with(|admin_data| admin_data.borrow().get(&payload.admin_id)) {
        Some(mut admin_post)=>{
            admin_post.admin_access = payload.admin_access;
            admin_post.admin_name = payload.admin_name;
            add_admin_data(&admin_post);
            Ok(admin_post)
        }
        None => Err(Error::NotFound { msg: format!(" can not update admin post with {} id  ", payload.admin_id) })
    }
}



#[update]
fn delete_user_post(id: u64) -> Result<UserArg, Error> {
    match STORAGE.with(|data| data.borrow_mut().remove(&id)) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("can not delete post with {} id", id),
        }),
    }
}

#[update]
fn delete_admin_data(id : u64)-> Result<AdminArg, Error>{
    match ADMIN_STORAGE.with(|data| data.borrow_mut().remove(&id)) {
        Some(msg)=> Ok(msg),
        None => Err(Error::NotFound { msg: format!(" can not delete admin data with {} id ", id),
     })
    }
}

ic_cdk::export_candid!();
