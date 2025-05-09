use crate::{
    dto::channel::{
        Channel as ChanDto, CreateChannelRequest, CreateChannelResp, GetChanResp, JoinChanResp,
        ListChanReq, ListChanResp,
    },
    errors::AppError,
    models::{
        channel::{ChanRepository, CreateChannel},
        user::UserRepository,
    },
};

pub struct ChannelService<'a> {
    chan_store: &'a ChanRepository<'a>,
    user_store: &'a UserRepository<'a>,
}

impl<'a> ChannelService<'a> {
    pub fn new(chan_store: &'a ChanRepository, user_store: &'a UserRepository<'a>) -> Self {
        Self {
            chan_store,
            user_store,
        }
    }

    pub async fn create_channel(
        &self,
        req: &CreateChannelRequest,
    ) -> Result<CreateChannelResp, AppError> {
        let user = self.user_store.get_by_id(req.creator_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound(format!(
                "user: {} not found",
                req.creator_id
            )));
        }

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

    pub async fn list_channels(&self, req: &ListChanReq) -> Result<ListChanResp, AppError> {
        let chan_list = self.chan_store.list_all(req.creator_id).await?;
        Ok(ListChanResp {
            channels: chan_list.into_iter().map(|ch| ChanDto::from(ch)).collect(),
        })
    }

    pub async fn get_channel(&self, channel_id: i64) -> Result<GetChanResp, AppError> {
        let channel = self.chan_store.get_by_id(channel_id).await?;
        if channel.is_none() {
            return Ok(GetChanResp { channel: None });
        }

        Ok(GetChanResp {
            channel: Some(ChanDto::from(channel.unwrap())),
        })
    }

    pub async fn join_channel(
        &self,
        user_id: i64,
        channel_id: i64,
    ) -> Result<JoinChanResp, AppError> {
        let channel = self.chan_store.get_by_id(channel_id).await?;
        if channel.is_none() {
            return Err(AppError::NotFound("channel not found".to_string()));
        }

        let user = self.user_store.get_by_id(user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound(format!("user: {} not found", user_id)));
        }

        let chan_member = self
            .chan_store
            .add_channel_member(channel_id, user_id)
            .await?;

        Ok(JoinChanResp { chan_member })
    }
}
