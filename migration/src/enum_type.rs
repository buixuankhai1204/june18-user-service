// use crate::sea_orm::prelude::*;
use sea_orm_migration::prelude::*;
use std::fmt::Write;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Status {
    Status,
    Active,
    Inactive,
}

impl Iden for Status {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Active => "active",
                Self::Inactive => "inactive",
                _ => {
                    "status"
                },
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ApproveState {
    ApproveState,
    Rejected,
    Approved,
    Pending,
    Cancelled,
    InProgress,
}

impl Iden for ApproveState {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Rejected => "rejected",
                Self::Approved => "approved",
                Self::Pending => "pending",
                Self::Cancelled => "cancelled",
                Self::InProgress => "in_progress",
                _ => {
                    "approve_state"
                },
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TypeOfRepeat {
    TypeOfRepeat,
    NoRepeat,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    WeeklyOn,
}

impl Iden for TypeOfRepeat {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::NoRepeat => "no_repeat",
                Self::Hourly => "hourly",
                Self::Daily => "daily",
                Self::Weekly => "weekly",
                Self::Monthly => "monthly",
                Self::WeeklyOn => "weekly_on",
                _ => {
                    "type_of_repeat"
                },
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TypeOfProgram {
    TypeOfProgram,
    Program,
    SocializeProgram,
    AdvertisementProgram,
}

impl Iden for TypeOfProgram {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Program => "program",
                Self::SocializeProgram => "socialize_program",
                Self::AdvertisementProgram => "advertisement_program",
                _ => {
                    "type_of_program"
                },
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TypeOfApproval {
    TypeOfApproval,
    Frame,
    CutAdvertisement,
    Program,
    ProgramSchedule,
}

impl Iden for TypeOfApproval {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Frame => "frame",
                Self::CutAdvertisement => "cut_advertisement",
                Self::Program => "program",
                Self::ProgramSchedule => "program_schedule",
                _ => {
                    "type_of_approval"
                },
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum State {
    State,
    Approve,
    Reject,
    Pending,
    Process,
    Cancel,
    FirstStep,
    SecondStep,
}

impl Iden for State {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Approve => "approve",
                Self::Reject => "reject",
                Self::Pending => "pending",
                Self::Process => "process",
                Self::Cancel => "cancel",
                Self::FirstStep => "first_step",
                Self::SecondStep => "second_step",
                _ => {
                    "state"
                },
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TypeOfForm {
    TypeOfForm,
    Frame,
    Program,
    Playlist,
}

impl Iden for TypeOfForm {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Frame => "frame",
                Self::Program => "program",
                Self::Playlist => "playlist",
                _ => {
                    "type_of_form"
                },
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditRequestType {
    EditRequestType,
    ChangeInformation,
    Remove,
    Swap,
    ScheduleSlot,
    ContentUpdate,
    ScheduleChange,
    MetadataModification,
    ConfigurationChange,
    BulkOperation,
    Emergency,
}

impl Iden for EditRequestType {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::ChangeInformation => "change_information",
                Self::Remove => "remove",
                Self::Swap => "swap",
                Self::ScheduleSlot => "schedule_slot",
                Self::ContentUpdate => "content_update",
                Self::ScheduleChange => "schedule_change",
                Self::MetadataModification => "metadata_modification",
                Self::ConfigurationChange => "configuration_change",
                Self::BulkOperation => "bulk_operation",
                Self::Emergency => "emergency",
                _ => "type_of_request",
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditRequestStatus {
    EditRequestStatus,
    Pending,
    InReview,
    Approved,
    AwaitingRequesterApproval,
    Rejected,
    Completed,
    Cancelled,
}

impl Iden for EditRequestStatus {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Pending => "pending",
                Self::InReview => "in_review",
                Self::Approved => "approved",
                Self::AwaitingRequesterApproval => "awaiting_requester_approval",
                Self::Rejected => "rejected",
                Self::Completed => "completed",
                Self::Cancelled => "cancelled",
                _ => "edit_request_status",
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditRequestPriority {
    EditRequestPriority,
    Low,
    Normal,
    High,
    Critical,
    Emergency,
}

impl Iden for EditRequestPriority {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Low => "low",
                Self::Normal => "normal",
                Self::High => "high",
                Self::Critical => "critical",
                Self::Emergency => "emergency",
                _ => "priority",
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ApprovalUnitStatus {
    ApprovalUnitStatus,
    Active,
    Inactive,
    Pending,
    Suspended,
}

impl Iden for ApprovalUnitStatus {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Active => "active",
                Self::Inactive => "inactive",
                Self::Pending => "pending",
                Self::Suspended => "suspended",
                _ => "approval_unit_status",
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PositionType {
    PositionType,
    Front,
    Middle,
    Behind,
}

impl Iden for PositionType {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Front => "front",
                Self::Middle => "middle",
                Self::Behind => "behind",
                _ => "position_type",
            }
        )
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TypeOfDate {
    TypeOfDate,
    Morning,
    Afternoon,
    Night,
    Midnight,
}

impl Iden for TypeOfDate {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Morning => "morning",
                Self::Afternoon => "afternoon",
                Self::Night => "night",
                Self::Midnight => "midnight",
                _ => "type_of_date",
            }
        )
        .unwrap();
    }
}
