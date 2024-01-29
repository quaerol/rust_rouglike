use specs::{saveload::SimpleMarker, World};

use crate::SerializeMe;

// serialize_individually 宏解决 Serde 和 Specs 之间进行协同工作的问题，如何将Specs 中的Component　类型序列化
macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

//
pub fn save_game(ecs: &mut World) {
    // 复制一份地图
    let mapcopy = ecs.get_mut::<super::map::Map>().unwrap().clone();

    let mapcopy = ecs.get_mut::<super::map::Map>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper { map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
