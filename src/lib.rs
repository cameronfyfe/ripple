#![no_std]


pub enum AudioFormat {
    PCM,
    IEEEFloat,
    EightBitALaw,
    EightBitMuLaw
}


impl AudioFormat {
    pub fn code(&self) -> u8 {
        match *self {
            AudioFormat::PCM           => 1,
            AudioFormat::IEEEFloat     => 3,
            AudioFormat::EightBitALaw  => 6,
            AudioFormat::EightBitMuLaw => 7
        }
    }
}


pub struct FileInfo {
    file_size: u32,      // total file size
    data_size: u32,      // size of audio data
    data_offset: u32,    // offset of start of audio data in file
    format: AudioFormat, // format type. 1-PCM, 3-IEEE float, 6-8bit A law, 7-8bit mu law
    num_channels: u8,    // no.of channels
    sample_rate: u32,    // sampling rate (blocks per second)
    bits_per_sample: u8  // bits per sample, 8- 8bits, 16- 16 bits etc
}


impl Default for FileInfo {
    fn default () -> FileInfo {
        FileInfo {
            file_size: 0,
            data_size: 0,
            data_offset: 0,
            format: AudioFormat::Invalid,
            num_channels: 0,
            sample_rate: 0,
            bits_per_sample: 0
        }
    }
}


pub fn process_header(header_bytes: &[u8]) -> Result<FileInfo, i32> {
    
    let mut file_info = FileInfo::default();

    // TODO

    Ok(file_info)
}


// TODO: everything else
