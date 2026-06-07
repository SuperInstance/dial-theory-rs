//! # dial-theory-rs
//!
//! Cultural dial positions for agent personality — where agents fall on theoretical spectrums.
//!
//! Provides:
//! - **position**: DialPosition with x,y coordinates on a 2D cultural dial
//! - **tradition**: Cultural traditions with positions on dials
//! - **distance**: Distance metrics between traditions
//! - **clustering**: Cluster traditions by proximity
//! - **evolution**: How traditions evolve over time

pub mod position;
pub mod tradition;
pub mod distance;
pub mod clustering;
pub mod evolution;
