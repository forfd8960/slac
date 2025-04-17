use crate::models::channel::ChanRepository;

pub struct ChannelService<'a> {
    chan_store: &'a ChanRepository<'a>,
}

impl<'a> ChannelService<'a> {
    pub fn new(chan_store: &'a ChanRepository) -> Self {
        Self { chan_store }
    }
}
