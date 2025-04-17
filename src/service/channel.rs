use crate::{
    dto::channel::{Channel as ChanDto, CreateChannelRequest, CreateChannelResp},
    errors::AppError,
    models::channel::{ChanRepository, CreateChannel},
};

pub struct ChannelService<'a> {
    chan_store: &'a ChanRepository<'a>,
}

impl<'a> ChannelService<'a> {
    pub fn new(chan_store: &'a ChanRepository) -> Self {
        Self { chan_store }
    }

    pub async fn create_channel(
        &self,
        req: &CreateChannelRequest,
    ) -> Result<CreateChannelResp, AppError> {
        let ch = CreateChannel {
            ch_name: req.ch_name.clone(),
            ch_description: req.ch_desc.clone(),
            creator_id: req.creator_id,
            is_private: req.is_private,
            is_archived: false,
        };

        let ch_result = self.chan_store.create(&ch).await?;

        Ok(CreateChannelResp {
            channel: ChanDto::from(ch_result),
        })
    }
}
