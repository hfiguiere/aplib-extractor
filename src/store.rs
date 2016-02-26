


use folder::Folder;
use album::Album;
use version::Version;
use master::Master;
use ::AplibObject;

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
