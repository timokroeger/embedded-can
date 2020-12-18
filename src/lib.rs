//! Controller Area Network
#![no_std]

pub mod blocking;

mod id;

pub use id::*;

/// A CAN2.0 Frame
pub trait Frame: Sized {
    /// Creates a new frame.
    /// Returns an error when the data slice is too long.
    fn new(id: impl Into<Id>, data: &[u8]) -> Result<Self, ()>;

    /// Creates a new remote frame (RTR bit set).
    /// Returns an error when the data length code (DLC) is not valid.
    fn new_remote(id: impl Into<Id>, dlc: usize) -> Result<Self, ()>;

    /// Returns true if this frame is a extended frame.
    fn is_extended(&self) -> bool;

    /// Returns true if this frame is a standard frame.
    fn is_standard(&self) -> bool {
        !self.is_extended()
    }

    /// Returns true if this frame is a remote frame.
    fn is_remote_frame(&self) -> bool;

    /// Returns true if this frame is a data frame.
    fn is_data_frame(&self) -> bool {
        !self.is_remote_frame()
    }

    /// Returns the frame identifier.
    fn id(&self) -> Id;

    /// Returns the data length code (DLC) which is in the range 0..8.
    ///
    /// For data frames the DLC value always matches the length of the data.
    /// Remote frames do not carry any data, yet the DLC can be greater than 0.
    fn dlc(&self) -> usize;

    /// Returns the frame data (0..8 bytes in length).
    fn data(&self) -> &[u8];
}

/// A CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: Frame;

    /// Associated error type.
    type Error;

    /// Puts a frame in the transmit buffer to be sent on the bus.
    ///
    /// If the transmit buffer is full, this function will try to replace a pending
    /// lower priority frame and return the frame that was replaced.
    /// Returns `Err(WouldBlock)` if the transmit buffer is full and no frame can be
    /// replaced.
    ///
    /// # Notes for implementers
    ///
    /// * Frames of equal identifier shall be transmited in FIFO fashion when more
    ///   than one transmit buffer is available.
    /// * When replacing pending frames make sure the frame is not in the process of
    ///   being send to the bus.
    fn try_transmit(&mut self, frame: &Self::Frame)
        -> nb::Result<Option<Self::Frame>, Self::Error>;

    /// Returns a received frame if available.
    fn try_receive(&mut self) -> nb::Result<Self::Frame, Self::Error>;
}
