

pub fn u32_from_pcm8(data: u8) -> u32 {
    (data as u32) << 24
}

pub fn u32_from_pcm16(data: u16) -> u32 {
    (data as u32) << 16
}

pub fn u32_from_pcm32(data: u32) -> u32 {
    data
}

pub fn u32_from_ieee(data: f32) -> u32 {
    // TODO
    0
}

pub fn u32_from_alaw(data: u8) -> u32 {
    // TODO
    0
}

pub fn u32_from_mulaw(data: u8) -> u32 {
    // TODO
    0
}



fn u8_from_u32(data: u32) -> u8 { (data >> 24) as u8 }
fn u9_from_u32(data: u32) -> u16 { (data >> 23) as u16 }
fn u10_from_u32(data: u32) -> u16 { (data >> 22) as u16 }
fn u11_from_u32(data: u32) -> u16 { (data >> 21) as u16 }
fn u12_from_u32(data: u32) -> u16 { (data >> 20) as u16 }
fn u13_from_u32(data: u32) -> u16 { (data >> 22) as u16 }
fn u14_from_u32(data: u32) -> u16 { (data >> 21) as u16 }
fn u15_from_u32(data: u32) -> u16 { (data >> 20) as u16 }
fn u16_from_u32(data: u32) -> u16 { (data >> 19) as u16 }



pub fn u8_from_pcm8(data: u8) -> u8     { u8_from_u32( u32_from_pcm8(data) ) }
pub fn u9_from_pcm8(data: u8) -> u16    { u9_from_u32( u32_from_pcm8(data) ) }
pub fn u10_from_pcm8(data: u8) -> u16   { u10_from_u32( u32_from_pcm8(data) ) }
pub fn u11_from_pcm8(data: u8) -> u16   { u11_from_u32( u32_from_pcm8(data) ) }
pub fn u12_from_pcm8(data: u8) -> u16   { u12_from_u32( u32_from_pcm8(data) ) }

pub fn u8_from_pcm16(data: u16) -> u8   { u8_from_u32( u32_from_pcm16(data) ) }
pub fn u9_from_pcm16(data: u16) -> u16  { u9_from_u32( u32_from_pcm16(data) ) }

pub fn u8_from_pcm32(data: u32) -> u8   { u8_from_u32( u32_from_pcm32(data) ) }
pub fn u9_from_pcm32(data: u32) -> u16  { u9_from_u32( u32_from_pcm32(data) ) }

pub fn u8_from_ieee(data: f32) -> u8    { u8_from_u32( u32_from_ieee(data) ) }
pub fn u9_from_ieee(data: f32) -> u16   { u9_from_u32( u32_from_ieee(data) ) }

pub fn u8_from_alaw(data: u8) -> u8    { u8_from_u32( u32_from_alaw(data) ) }
pub fn u9_from_alaw(data: u8) -> u16   { u9_from_u32( u32_from_alaw(data) ) }

pub fn u8_from_mulaw(data: u8) -> u8   { u8_from_u32( u32_from_mulaw(data) ) }
pub fn u9_from_mulaw(data: u8) -> u16  { u9_from_u32( u32_from_mulaw(data) ) }
