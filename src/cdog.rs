#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
    pub control: CONTROL,
    #[doc = "0x04 - Instruction timer reload."]
    pub reload: RELOAD,
    #[doc = "0x08 - The INSTRUCTION TIMER itself."]
    pub instruction_timer: INSTRUCTION_TIMER,
    #[doc = "0x0c - Also known as SEC_CNT."]
    pub secure_counter: SECURE_COUNTER,
    #[doc = "0x10 - Status register (1 of 2)"]
    pub status: STATUS,
    #[doc = "0x14 - STATUS register (2 of 2)"]
    pub status2: STATUS2,
    #[doc = "0x18 - Hardware flags."]
    pub flags: FLAGS,
    #[doc = "0x1c - Persistent (Ad. Hoc., quasi-NV) data storage."]
    pub persistent: PERSISTENT,
    #[doc = "0x20 - Write address for issuing the START command."]
    pub start: START,
    #[doc = "0x24 - Write address for issuing the STOP command."]
    pub stop: STOP,
    #[doc = "0x28 - Write address for issuing the RESTART command."]
    pub restart: RESTART,
    #[doc = "0x2c - Write address for issuing the ADD command."]
    pub add: ADD,
    #[doc = "0x30 - Write address for issuing the ADD1 command."]
    pub add1: ADD1,
    #[doc = "0x34 - Write address for issuing the ADD16 command."]
    pub add16: ADD16,
    #[doc = "0x38 - Write address for issuing the ADD16 command."]
    pub add256: ADD256,
    #[doc = "0x3c - Write address for issuing the SUB command."]
    pub sub: SUB,
    #[doc = "0x40 - Write address for issuing the SUB1 command."]
    pub sub1: SUB1,
    #[doc = "0x44 - Write address for issuing the SUB16 command."]
    pub sub16: SUB16,
    #[doc = "0x48 - Write address for issuing the SUB256 command."]
    pub sub256: SUB256,
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
pub mod control;
#[doc = "RELOAD (rw) register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "Instruction timer reload."]
pub mod reload;
#[doc = "INSTRUCTION_TIMER (rw) register accessor: an alias for `Reg<INSTRUCTION_TIMER_SPEC>`"]
pub type INSTRUCTION_TIMER = crate::Reg<instruction_timer::INSTRUCTION_TIMER_SPEC>;
#[doc = "The INSTRUCTION TIMER itself."]
pub mod instruction_timer;
#[doc = "SECURE_COUNTER (rw) register accessor: an alias for `Reg<SECURE_COUNTER_SPEC>`"]
pub type SECURE_COUNTER = crate::Reg<secure_counter::SECURE_COUNTER_SPEC>;
#[doc = "Also known as SEC_CNT."]
pub mod secure_counter;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register (1 of 2)"]
pub mod status;
#[doc = "STATUS2 (r) register accessor: an alias for `Reg<STATUS2_SPEC>`"]
pub type STATUS2 = crate::Reg<status2::STATUS2_SPEC>;
#[doc = "STATUS register (2 of 2)"]
pub mod status2;
#[doc = "FLAGS (rw) register accessor: an alias for `Reg<FLAGS_SPEC>`"]
pub type FLAGS = crate::Reg<flags::FLAGS_SPEC>;
#[doc = "Hardware flags."]
pub mod flags;
#[doc = "PERSISTENT (rw) register accessor: an alias for `Reg<PERSISTENT_SPEC>`"]
pub type PERSISTENT = crate::Reg<persistent::PERSISTENT_SPEC>;
#[doc = "Persistent (Ad. Hoc., quasi-NV) data storage."]
pub mod persistent;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Write address for issuing the START command."]
pub mod start;
#[doc = "STOP (w) register accessor: an alias for `Reg<STOP_SPEC>`"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = "Write address for issuing the STOP command."]
pub mod stop;
#[doc = "RESTART (w) register accessor: an alias for `Reg<RESTART_SPEC>`"]
pub type RESTART = crate::Reg<restart::RESTART_SPEC>;
#[doc = "Write address for issuing the RESTART command."]
pub mod restart;
#[doc = "ADD (w) register accessor: an alias for `Reg<ADD_SPEC>`"]
pub type ADD = crate::Reg<add::ADD_SPEC>;
#[doc = "Write address for issuing the ADD command."]
pub mod add;
#[doc = "ADD1 (w) register accessor: an alias for `Reg<ADD1_SPEC>`"]
pub type ADD1 = crate::Reg<add1::ADD1_SPEC>;
#[doc = "Write address for issuing the ADD1 command."]
pub mod add1;
#[doc = "ADD16 (w) register accessor: an alias for `Reg<ADD16_SPEC>`"]
pub type ADD16 = crate::Reg<add16::ADD16_SPEC>;
#[doc = "Write address for issuing the ADD16 command."]
pub mod add16;
#[doc = "ADD256 (w) register accessor: an alias for `Reg<ADD256_SPEC>`"]
pub type ADD256 = crate::Reg<add256::ADD256_SPEC>;
#[doc = "Write address for issuing the ADD16 command."]
pub mod add256;
#[doc = "SUB (w) register accessor: an alias for `Reg<SUB_SPEC>`"]
pub type SUB = crate::Reg<sub::SUB_SPEC>;
#[doc = "Write address for issuing the SUB command."]
pub mod sub;
#[doc = "SUB1 (w) register accessor: an alias for `Reg<SUB1_SPEC>`"]
pub type SUB1 = crate::Reg<sub1::SUB1_SPEC>;
#[doc = "Write address for issuing the SUB1 command."]
pub mod sub1;
#[doc = "SUB16 (w) register accessor: an alias for `Reg<SUB16_SPEC>`"]
pub type SUB16 = crate::Reg<sub16::SUB16_SPEC>;
#[doc = "Write address for issuing the SUB16 command."]
pub mod sub16;
#[doc = "SUB256 (w) register accessor: an alias for `Reg<SUB256_SPEC>`"]
pub type SUB256 = crate::Reg<sub256::SUB256_SPEC>;
#[doc = "Write address for issuing the SUB256 command."]
pub mod sub256;
