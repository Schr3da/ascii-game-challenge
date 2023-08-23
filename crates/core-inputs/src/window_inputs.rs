use core_dtos::prelude::*;
use core_state::prelude::*;

pub struct WindowInputs;

impl WindowInputs {
    pub async fn handle_event(event: WindowEvents, app_state: &mut AppState) -> bool {
        match event {
            WindowEvents::Resize(w, h) => {
                app_state
                    .send(SendEvents::General(GeneralEvents::OnApplicationResize(
                        w, h,
                    )))
                    .await
            }
        };

        true
    }
}
