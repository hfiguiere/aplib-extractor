


use aplib::folder::Folder;
use aplib::album::Album;
use aplib::version::Version;
use aplib::master::Master;
use aplib::AplibObject;

pub enum Wrapper {
    Album(Box<Album>),
    Folder(Box<Folder>),
    Master(Box<Master>),
    Version(Box<Version>),
    None
}

impl Wrapper {
    pub fn uuid(&self) -> Option<&String> {
        return match *self {
            Wrapper::Album(ref o) => {
                Some(o.uuid())
            },
            Wrapper::Folder(ref o) => {
                Some(o.uuid())
            },
            Wrapper::Version(ref o) => {
                Some(o.uuid())
            },
            Wrapper::Master(ref o) => {
                Some(o.uuid())
            },
            Wrapper::None => Option::None
        };
    }
}
