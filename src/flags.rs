use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PacketFlags: u16 {
        const None = 0;
        const CarOnTrack = 1 << 0;
        const Paused = 1 << 1;
        const LoadingOrProcessing = 1 << 2;
        const InGear = 1 << 3;
        const HasTurbo = 1 << 4;
        const RevLimiterBlinkAlertActive = 1 << 5;
        const HandBrakeActive = 1 << 6;
        const LightsActive = 1 << 7;
        const HighBeamActive = 1 << 8;
        const LowBeamActive = 1 << 9;
        const ASMActive = 1 << 10;
        const TCSActive = 1 << 11;
    }
}