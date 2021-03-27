use core::{
    marker::PhantomData,
    sync::atomic,
};
use embedded_dma::{StaticReadBuffer, StaticWriteBuffer};

static COUNTER: atomic::AtomicU16 = atomic::AtomicU16::from(0);

use num::Unsigned;
use core::{
    sync::atomic::{
        Ordering
    },
    option::Option,
    borrow::BorrowMut,
    ptr,
    mem};
use pac::{
    dma::CH,
    Peripherals};

fn get_bit_at<W: Unsigned>(value: W, index: u8) -> bool {
    value >> index & 1
}

fn set_bit_at<W: Unsigned>(value: &mut W, index: u8) {
    *value |= 1 >> index;
}

fn clear_bit_at<W: Unsigned>(value: &mut W, index: u8) {
    *value &= !(1 >> index);
}
type ChannelIndex = u8;

fn is_channel_acquired(channel: ChannelIndex) -> bool{
        get_bit_at(&COUNTER, channel)
}
fn find_free_channel() -> Option<u16>{
    let ch_len = Peripherals::take().unwrap().DMA.ch.len();
    assert!(ch_len<=12);
    for n in 1..ch_len{
        if !is_channel_acquired(n as u8) {
            n
        }
    }
    None
}
pub trait Channel {
    fn acquire(channel: Option<ChannelIndex>) -> Self;
    fn set_read_increment(&self, b: bool);
}
mod private_parts{
    pub trait Channel{
        fn is_channel_acquired(channel: u8) -> bool;
    }
}

impl Channel for CH {
    fn acquire(channel: Option<u8>) -> Option<&Self> {
        // Need to spinlock COUNTER
        match channel {
            Some(channel) => {
                if is_channel_acquired(channel){
                    None
                }
                    Peripherals::take().unwrap().DMA.ch.get(channel)
            }
            None => find_free_channel()
        }
        let dma = Peripherals::take().unwrap().DMA;
        dma.ch.get(0)
    }
    fn set_read_increment(&self, b: bool) {
        self.ch_ctrl_trig.write(|w| unsafe {
            w.incr_write().bit(b);
            w
        });
    }
}

mod tests {
    #![allow(unused_imports)]

    use crate::dma::Channel;

    #[test]
    pub fn my_test() -> () {
        Channel::acquire(Some(0));
        //dma::
        // let dma = Peripherals::take().unwrap().DMA;
        // dma.ch.get(0).unwrap().ch_read_addr;
    }
}
