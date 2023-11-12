use std::env;

use axum::extract::State;
use sea_orm::DatabaseConnection;
use web_push::WebPushClient;

use crate::app::AppState;

use super::repo::PushSubscriptionsRepo;

mod create;
mod vapid;

pub struct PushSubscriptionsController {}

impl PushSubscriptionsController {
    pub async fn test(State(state): State<AppState>) {
        let db: &DatabaseConnection = state.db.as_ref();

        let ps = PushSubscriptionsRepo::find_all(db).await;

        let qq = ps.first().unwrap();

        let si = web_push::SubscriptionInfo::new(
            qq.endpoint.clone(),
            qq.p256dh_key.clone(),
            qq.auth_key.clone(),
        );

        let mut wpmb = web_push::WebPushMessageBuilder::new(&si);

        let sig = web_push::VapidSignatureBuilder::from_base64(
            &env::var("VAPID_PRIVATE_KEY").unwrap(),
            web_push::URL_SAFE_NO_PAD,
            &si,
        )
        .unwrap();

        wpmb.set_payload(web_push::ContentEncoding::Aes128Gcm, "QQQ".as_bytes());
        wpmb.set_vapid_signature(sig.build().unwrap());

        let client = web_push::IsahcWebPushClient::new().unwrap();
        client.send(wpmb.build().unwrap()).await.unwrap();
    }
}
