mod demo_repository;
mod live_repository;

pub(crate) use demo_repository::{CompanionRepository, MockCompanionRepository};
pub(crate) use live_repository::LiveCompanionRepository;