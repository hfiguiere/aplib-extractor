


use aplib::folder::Folder;
use aplib::album::Album;
use aplib::version::Version;
use aplib::master::Master;
use aplib::AplibObject;

pub enum ObjectStoreWrapper {
    Album(Box<Album>),
    Folder(Box<Folder>),
    Master(Box<Master>),
    Version(Box<Version>),
    None
}

impl ObjectStoreWrapper {
    pub fn uuid(&self) -> Option<&String> {
        return match *self {
            ObjectStoreWrapper::Album(ref o) => {
                Some(o.uuid())
            },
            ObjectStoreWrapper::Folder(ref o) => {
                Some(o.uuid())
            },
            ObjectStoreWrapper::Version(ref o) => {
                Some(o.uuid())
            },
            ObjectStoreWrapper::Master(ref o) => {
                Some(o.uuid())
            },
            ObjectStoreWrapper::None => Option::None
        };
    }
}
