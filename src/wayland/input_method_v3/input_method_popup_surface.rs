use std::sync::{Arc, Mutex};

use wl_input_method::input_method::xx::server::xx_input_popup_surface_v2::{
    self, XxInputPopupSurfaceV2,
};
use wayland_server::{backend::ClientId, protocol::wl_surface::WlSurface, Dispatch, Resource};

use crate::utils::{
    alive_tracker::{AliveTracker, IsAlive},
    Logical, Point, Rectangle,
};


use wl_input_method::input_method::xx::server::xx_input_popup_surface_v2::{
    self as zwp_input_popup_surface_v2, XxInputPopupSurfaceV2 as ZwpInputPopupSurfaceV2,
};

use super::InputMethodManagerState;

/// Handle to a popup surface
#[derive(Debug, Clone, Default)]
pub struct PopupHandle {
    pub surface: Option<PopupSurface>,
    pub rectangle: Rectangle<i32, Logical>,
}

#[derive(Debug, Clone)]
pub struct ImPopupLocation {
    /// Area of text that should not be covered, relative to parent
    pub cursor: Rectangle<i32, Logical>,
    /// Location of the popup surface relative to parent.
    pub location: Point<i32, Logical>,
}

/// A handle to an input method popup surface
#[derive(Debug, Clone)]
pub struct PopupSurface {
    /// The surface role for the input method popup
    pub surface_role: ZwpInputPopupSurfaceV2,
    surface: WlSurface,
    /// Positioning information. None if popup not mapped.
    position: Arc<Mutex<Option<ImPopupLocation>>>,
    /// Current parent of the IME popup.
    /// A popup may have a parent without a position while it's waiting for `set_cursor_rectangle`.
    parent: Option<PopupParent>,
}

impl PopupSurface {
    /// Creates a new unmapped popup surface
    pub(crate) fn new(
        surface_role: ZwpInputPopupSurfaceV2,
        surface: WlSurface,
        parent: Option<PopupParent>,
    ) -> Self {
        Self {
            surface_role,
            position: Arc::new(Mutex::new(None)),
            surface,
            parent,
        }
    }

    /// Is the input method popup surface referred by this handle still alive?
    #[inline]
    pub fn alive(&self) -> bool {
        // TODO other things to check? This may not sufice.
        let role_data: &InputMethodPopupSurfaceUserData = self.surface_role.data().unwrap();
        self.surface.alive() && role_data.alive_tracker.alive()
    }

    /// Access to the underlying `wl_surface` of this popup
    #[inline]
    pub fn wl_surface(&self) -> &WlSurface {
        &self.surface
    }

    /// Access to the parent surface associated with this popup
    pub fn get_parent(&self) -> Option<&PopupParent> {
        self.parent.as_ref()
    }

    /// Set the IME popup surface parent.
    pub fn set_parent(&mut self, parent: Option<PopupParent>) {
        self.parent = parent;
    }

    /// Used to access the location of an input popup surface relative to the parent
    pub fn location(&self) -> Option<Point<i32, Logical>> {
        //self.position.as_ref().map(|p| p.lock().unwrap().location)
        self.position.lock().unwrap().as_ref().map(|p| p.location)
    }

    /// The region compositor shouldn't obscure when placing the popup within the
    /// client.
    pub fn cursor_rectangle(&self) -> Option<Rectangle<i32, Logical>> {
        self.position.lock().unwrap().as_ref().map(|p| p.cursor)
    }

    /// Set position information that should take effect when mapping.
    ///
    /// This issues the `text_input_rectangle` event on the popup object.
    pub fn set_position(&mut self, position: Option<ImPopupLocation>) {
        if let Some(ImPopupLocation { cursor, location }) = &position {
            let relative_to_popup = cursor.loc - *location;
            self.surface_role.text_input_rectangle(
                relative_to_popup.x,
                relative_to_popup.y,
                cursor.size.w,
                cursor.size.h,
            );
        }
        *self.position.lock().unwrap() = position;
    }
}

impl std::cmp::PartialEq for PopupSurface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.surface_role == other.surface_role
    }
}

/// Parent surface and location for the IME popup.
#[derive(Debug, Clone)]
pub struct PopupParent {
    /// The surface IME popup is present over.
    pub surface: WlSurface,
    /// The location of the parent surface.
    pub location: Rectangle<i32, Logical>,
}

/// User data of ZwpInputPopupSurfaceV2 object
#[derive(Debug)]
pub struct InputMethodPopupSurfaceUserData {
    pub(super) alive_tracker: AliveTracker,
}

impl<D> Dispatch<ZwpInputPopupSurfaceV2, InputMethodPopupSurfaceUserData, D> for InputMethodManagerState {
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _resource: &ZwpInputPopupSurfaceV2,
        request: zwp_input_popup_surface_v2::Request,
        _data: &InputMethodPopupSurfaceUserData,
        _dhandle: &wayland_server::DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            zwp_input_popup_surface_v2::Request::Destroy => {
                // Nothing to do
            }
            _ => unreachable!(),
        }
    }

    fn destroyed(
        _state: &mut D,
        _client: ClientId,
        _object: &ZwpInputPopupSurfaceV2,
        data: &InputMethodPopupSurfaceUserData,
    ) {
        data.alive_tracker.destroy_notify();
    }
}
