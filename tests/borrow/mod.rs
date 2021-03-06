#[cfg(feature = "non_send")]
use core::any::type_name;
use shipyard::error;
use shipyard::*;

#[test]
fn simple_borrow() {
    let world = World::new();

    let u32s = world.borrow::<View<u32>>();
    assert_eq!(u32s.len(), 0);
}

#[test]
fn all_storages_simple_borrow() {
    let world = World::new();

    let all_storages = world.borrow::<AllStoragesViewMut>();
    let u32s = all_storages.borrow::<View<u32>>();
    assert_eq!(u32s.len(), 0);
}

#[test]
fn invalid_borrow() {
    let world = World::new();

    let _u32s = world.borrow::<ViewMut<u32>>();
    assert_eq!(
        world.try_borrow::<ViewMut<u32>>().err(),
        Some(error::GetStorage::StorageBorrow((
            core::any::type_name::<u32>(),
            error::Borrow::Unique
        )))
    );
}

#[test]
fn all_storages_invalid_borrow() {
    let world = World::new();

    let all_storages = world.borrow::<AllStoragesViewMut>();
    let _u32s = all_storages.borrow::<ViewMut<u32>>();
    assert_eq!(
        all_storages.try_borrow::<ViewMut<u32>>().err(),
        Some(error::GetStorage::StorageBorrow((
            core::any::type_name::<u32>(),
            error::Borrow::Unique
        )))
    );
}

#[test]
fn double_borrow() {
    let world = World::new();

    let u32s = world.borrow::<ViewMut<u32>>();
    drop(u32s);
    world.borrow::<ViewMut<u32>>();
}

#[test]
fn all_storages_double_borrow() {
    let world = World::new();

    let all_storages = world.borrow::<AllStoragesViewMut>();
    let u32s = all_storages.borrow::<ViewMut<u32>>();
    drop(u32s);
    all_storages.borrow::<ViewMut<u32>>();
}

#[test]
#[cfg(feature = "non_send")]
fn non_send_storage_in_other_thread() {
    struct NonSendStruct(*const ());

    unsafe impl Sync for NonSendStruct {}

    let world = World::new();
    rayon::join(
        || {
            assert_eq!(
                world.try_borrow::<NonSend<ViewMut<NonSendStruct>>>().err(),
                Some(error::GetStorage::StorageBorrow((
                    type_name::<NonSendStruct>(),
                    error::Borrow::WrongThread
                )))
            )
        },
        || {},
    );
}

#[test]
#[cfg(all(feature = "non_send", feature = "non_sync"))]
fn non_send_sync_storage_in_other_thread() {
    struct NonSendSyncStruct(*const ());

    let world = World::new();
    rayon::join(
        || {
            assert_eq!(
                world
                    .try_borrow::<NonSendSync<View<NonSendSyncStruct>>>()
                    .err(),
                Some(error::GetStorage::StorageBorrow((
                    type_name::<NonSendSyncStruct>(),
                    error::Borrow::WrongThread
                )))
            )
        },
        || {},
    );
}
