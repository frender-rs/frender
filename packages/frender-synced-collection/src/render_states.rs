pub(super) enum MountState {
    MountedAndUpToDate,
    MountedAndUpToDateButPreviousWasSkipped,
    Outdated,
    OutdatedAndPreviousWasSkipped,
    OutdatedAndMoved,
    // UpdateToDateButMoved,
}

impl MountState {
    // caller should mark the next one as previous_was_skipped
    // There might be a UpdateToDateButMoved variant so this method is different from mark_as_outdated_and_moved
    pub(crate) fn mark_as_moved(&mut self) {
        *self = Self::OutdatedAndMoved
    }

    // caller should mark the next one as previous_was_skipped
    pub(crate) fn mark_as_outdated_and_moved(&mut self) {
        *self = Self::OutdatedAndMoved
    }

    pub(crate) fn mark_as_outdated(&mut self) {
        *self = match self {
            MountState::MountedAndUpToDate => Self::Outdated,
            MountState::Outdated => Self::Outdated,
            MountState::OutdatedAndMoved => MountState::OutdatedAndMoved,
            MountState::MountedAndUpToDateButPreviousWasSkipped => {
                MountState::OutdatedAndPreviousWasSkipped
            }
            MountState::OutdatedAndPreviousWasSkipped => MountState::OutdatedAndPreviousWasSkipped,
        }
    }

    pub(crate) fn mark_previous_was_skipped(&mut self) {
        match self {
            MountState::MountedAndUpToDate => {
                *self = MountState::MountedAndUpToDateButPreviousWasSkipped
            }
            MountState::Outdated => *self = MountState::OutdatedAndPreviousWasSkipped,
            _ => {}
        }
    }

    pub(crate) fn needs_reposition(&self) -> NeedsReposition {
        match self {
            MountState::MountedAndUpToDate => NeedsReposition::No {
                previous_skipped: false,
            },
            MountState::MountedAndUpToDateButPreviousWasSkipped => NeedsReposition::No {
                previous_skipped: true,
            },
            MountState::Outdated => NeedsReposition::No {
                previous_skipped: false,
            },
            MountState::OutdatedAndPreviousWasSkipped => NeedsReposition::No {
                previous_skipped: true,
            },
            MountState::OutdatedAndMoved => NeedsReposition::Yes,
        }
    }
}

pub(crate) enum NeedsReposition {
    Yes,
    No { previous_skipped: bool },
}

const _: () = assert!(std::mem::size_of::<NeedsReposition>() == 1);
