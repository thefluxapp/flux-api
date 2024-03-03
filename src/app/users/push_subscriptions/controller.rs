use serde::Serialize;

mod create;
mod vapid;

pub struct PushSubscriptionsController {}

// impl PushSubscriptionsController {
//     pub async fn test(State(state): State<AppState>) {
//         let db: &DatabaseConnection = state.db.as_ref();

//         let ps = PushSubscriptionsRepo::find_all(db).await;

//         for p in ps {
//             state
//                 .notifier
//                 .send(
//                     "title".to_string(),
//                     "body".to_string(),
//                     p.endpoint,
//                     p.p256dh_key,
//                     p.auth_key,
//                 )
//                 .await
//                 .unwrap();
//         }
//     }
// }

#[derive(Serialize)]
struct Payload {
    title: String,
    body: String,
}
