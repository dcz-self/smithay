//! Backend (rendering/input) creation helpers
//!
//! Collection of common traits and implementation about
//! rendering onto various targets and receiving input
//! from various sources.
//!
//! Supported graphics backends:
//!
//! - winit
//! - drm
//!
//! Supported input backends:
//!
//! - winit
//! - libinput

pub mod graphics;
pub mod input;

#[cfg(feature = "backend_drm")]
pub mod drm;
#[cfg(feature = "backend_egl")]
pub mod egl;
#[cfg(feature = "backend_libinput")]
pub mod libinput;
#[cfg(feature = "backend_session")]
pub mod session;
#[cfg(feature = "backend_udev")]
pub mod udev;
#[cfg(feature = "backend_winit")]
pub mod winit;
