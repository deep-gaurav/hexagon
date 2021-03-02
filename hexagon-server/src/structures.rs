
// #[derive(Debug, Clone, Serialize)]
// pub struct Player {
//     pub id: String,
//     pub name: String,
//     #[serde(skip)]
//     pub send_channel: UnboundedSender<Result<Message, warp::Error>>,
//     pub status: PlayerStatus,
// }


// impl Player {
//     pub fn send(&self, message: SocketMessage) {
//         match bincode::serialize(&message) {
//             Ok(bytes) => {
//                 if let Err(er) = self.send_channel.send(Ok(Message::binary(bytes))) {
//                     log::warn!("Cant send message to player {:#?} error {:#?}", self, er);
//                 }
//             }
//             Err(err) => {
//                 log::error!("Cant serialize {:#?} error {:#?}", message, err);
//             }
//         }
//     }
//     pub fn close(&self, code: CloseCodes) {
//         warn!("Closing connection to {:#?} code: {:#?}", &self, code);
//         if let Err(er) = self
//             .send_channel
//             .send(Ok(Message::close_with(code.to_code(), code.to_string())))
//         {
//             error!(
//                 "Cant send close message to player {:#?} error {:#?}",
//                 self, er
//             );
//         }
//     }
// }
