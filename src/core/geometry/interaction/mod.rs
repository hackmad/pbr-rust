//! Interactions

#![allow(dead_code)]
use crate::core::geometry::*;
use crate::core::medium::*;
use crate::core::pbrt::*;
use std::sync::Arc;

mod medium_interaction;
mod surface_interaction;

pub use medium_interaction::*;
pub use surface_interaction::*;

/// Interaction enumeration.
pub enum Interaction<'a> {
    /// Represents geometry of a particular point on a surface.
    Surface { si: SurfaceInteraction<'a> },

    /// Represents an interaction point in a scattering medium.
    Medium { mi: MediumInteraction },
}

impl<'a> Interaction<'a> {
    /// Returns the interaction hit point.
    pub fn get_hit(&self) -> &Hit {
        match self {
            Self::Surface { si } => &si.hit,
            Self::Medium { mi } => &mi.hit,
        }
    }
}

/// Hit provides common data shared by implementations of `Interaction` trait.
#[derive(Clone)]
pub struct Hit {
    /// Point of interaction.
    pub p: Point3f,

    /// Time when interaction occurred.
    pub time: Float,

    /// Floating point error for ray intersection points.
    pub p_error: Vector3f,

    /// The negative ray direction (outgoing direction used when computing
    /// lighting at points).
    pub wo: Vector3f,

    /// Surface normal at the point `p`.
    pub n: Normal3f,

    /// The medium interface used for scattering media.
    pub medium_interface: Option<MediumInterface>,
}

/// Atomic reference counted `Hit`.
type ArcHit = Arc<Hit>;

impl Hit {
    /// Create a new hit.
    ///
    /// `p`                - Point of interaction.
    /// `time`             - Time when interaction occurred.
    /// `p_error`          - Floating point error for ray intersection points.
    /// `wo`               - The negative ray direction (outgoing direction used
    ///                      when computing lighting at points).
    /// `n`                - Surface normal at the point `p`.
    /// `medium_interface` - The medium interface used for scattering media.
    pub fn new(
        p: Point3f,
        time: Float,
        p_error: Vector3f,
        wo: Vector3f,
        n: Normal3f,
        medium_interface: Option<MediumInterface>,
    ) -> Self {
        Self {
            p,
            time,
            p_error,
            wo,
            n,
            medium_interface,
        }
    }

    /// Returns `true` if this is a surface interaction.
    pub fn is_surface_interaction(&self) -> bool {
        self.n != Normal3f::zero()
    }

    /// Returns `true` if this is a medium interaction.
    pub fn is_medium_interaction(&self) -> bool {
        !self.is_surface_interaction()
    }

    /// Spawn's a new ray in the given direction.
    ///
    /// * `d` - The new direction.
    pub fn spawn_ray(&self, d: &Vector3f) -> Ray {
        let o = Ray::offset_origin(&self.p, &self.p_error, &self.n, d);
        Ray::new(o, *d, INFINITY, self.time, self.get_medium_in_direction(d))
    }

    /// Spawn's a new ray towards another point.
    ///
    /// * `p` - The target point.
    pub fn spawn_ray_to(&self, p: &Point3f) -> Ray {
        let dir = *p - self.p;
        let o = Ray::offset_origin(&self.p, &self.p_error, &self.n, &dir);
        let d = *p - o;
        Ray::new(
            o,
            d,
            1.0 - SHADOW_EPSILON,
            self.time,
            self.get_medium_in_direction(&d),
        )
    }

    /// Returns the medium towards a direction.
    ///
    /// * `w` - The direction.
    pub fn get_medium_in_direction(&self, w: &Vector3f) -> Option<ArcMedium> {
        let mi = self.medium_interface.clone();
        if w.dot(&self.n) > 0.0 {
            mi.map_or(None, |mi| mi.outside.clone())
        } else {
            mi.map_or(None, |mi| mi.inside.clone())
        }
    }

    /// Returns the medium when interior and exterior are the same.
    pub fn get_medium(&self) -> Option<ArcMedium> {
        if let Some(mi) = self.medium_interface.clone() {
            if mi.is_medium_transition() {
                mi.inside.clone()
            } else {
                None
            }
        } else {
            None
        }
    }
}
