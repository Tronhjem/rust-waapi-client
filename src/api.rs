/// Wwise Authoring API endpoints
pub mod ak {
    pub mod wwise {
        pub mod core {
            pub const GET_INFO: &str = "ak.wwise.core.getInfo";

            pub mod object {
                pub const GET: &str = "ak.wwise.core.object.get";
            }
        }
        pub mod ui {
            pub const GET_SELECTED_OBJECTS: &str = "ak.wwise.ui.getSelectedObjects";
        }
    }
}
