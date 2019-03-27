#![no_std]


mod util;
mod data;


pub enum AudioFormat {
    PCM,
    IEEEFloat,
    EightBitALaw,
    EightBitMuLaw,
    Invalid
}


impl AudioFormat {
    pub fn to_code(&self) -> u8 {
        match *self {
            AudioFormat::PCM           => 1,
            AudioFormat::IEEEFloat     => 3,
            AudioFormat::EightBitALaw  => 6,
            AudioFormat::EightBitMuLaw => 7,
            AudioFormat::Invalid       => 99
        }
    }

    pub fn from_code(code: u16) -> AudioFormat {
        match code {
            1 => AudioFormat::PCM,
            3 => AudioFormat::IEEEFloat,
            6 => AudioFormat::EightBitALaw,
            7 => AudioFormat::EightBitMuLaw,
            (_) => AudioFormat::Invalid
        }
    }
}





pub struct FileInfo {
    file_size: u32,      // total file size
    data_size: u32,      // size of audio data
    data_offset: u32,    // offset of start of audio data in file
    format: AudioFormat, // format type. 1-PCM, 3-IEEE float, 6-8bit A law, 7-8bit mu law
    num_channels: u16,   // no.of channels
    sample_rate: u32,    // sampling rate (blocks per second)
    bits_per_sample: u16 // bits per sample, 8- 8bits, 16- 16 bits etc
}


impl Default for FileInfo {
    fn default () -> FileInfo {
        FileInfo {
            file_size: 0,
            data_size: 0,
            data_offset: 0,
            format: AudioFormat::PCM,
            num_channels: 0,
            sample_rate: 0,
            bits_per_sample: 0
        }
    }
}


struct ParserInfo {
    file_info: FileInfo,
    fmt_chunk_processed: bool,
    data_chunk_processed: bool
}


// TODO: make this better, macro?
const FMT: u32  = 0x666d7420 as u32; // "fmt "
const FACT: u32 = 0x66616374 as u32; // "fact"
const DATA: u32 = 0x64617461 as u32; // "data"


enum ChunkType {
    Fmt,
    Fact,
    Data,
}


impl ChunkType {
    pub fn id(&self) -> u32 {
        match *self {
            // TODO: make this better, probably macro to convert from string
            ChunkType::Fmt  => FMT,
            ChunkType::Fact => FACT,
            ChunkType::Data => DATA
        }
    }
}


pub enum ParserError {
    BadFormat,
    BadConfig,
    OutOfBounds
}


pub fn process_header(bytes: &[u8]) -> Result<FileInfo, ParserError> {
    
    let mut parser = ParserInfo {
        file_info: FileInfo::default(),
        fmt_chunk_processed: false,
        data_chunk_processed: false
    };
    let mut i = 0 as usize;

    // Verify it's a RIFF file
    if util::u32_from_u8_slice(&bytes[0..3]) != util::u32_from_chars(['R','I','F','F']) {
        return Err(ParserError::BadFormat)
    }

    // Get file size
    parser.file_info.file_size = util::u32_from_u8_slice(&bytes[4..7]);

    // Verify it's a WAVE file
    if util::u32_from_u8_slice(&bytes[8..11]) != util::u32_from_chars(['W','A','V','E']) {
        return Err(ParserError::BadFormat)
    }

    i += 12;
    // Process chunks
    loop {
        // Chunk Structure:
        // bytes 0-3     : Chunk ID
        // bytes 4-7     : Chunk Size N
        // bytes 8-(N+8) : Chunk Data (Chunk ID/Size not included in chunk size)
        let chunk_id = util::u32_from_u8_slice(&bytes[i..i+3]);
        let chunk_size = util::u32_from_u8_slice(&bytes[i+4..i+7]) as usize;
        i += 8;

        // Make sure next chunk is in slice bounds
        if (i + chunk_size > bytes.len()) {
            return Err(ParserError::OutOfBounds);
        }

        // Get slice for just this chunk
        let chunk_bytes = &bytes[i..i+chunk_size];

        match chunk_id {
            FMT  => {
                // Format chunk with main info
                match process_format_chunk(&mut parser.file_info, &chunk_bytes) {
                    Ok(info) => parser.fmt_chunk_processed = true,
                    Err(error) => return Err(error)
                }
            },
            FACT => {
                // Ignore fact chunks
            },
            DATA => {
                // Add data chunk info to file info
                parser.file_info.data_size = chunk_size as u32;
                parser.file_info.data_offset = i as u32;
                parser.data_chunk_processed = true;

                // Quit checking after data chunk is encounterd
                // Parser assumes all chunks we need are before the data chunk
                break;
            },
            (_)  => {
                // Unknown chunk encountered
            }
        };

        i += chunk_size;
    }

    // Verify necessary chunks were processed
    if !parser.fmt_chunk_processed || !parser.data_chunk_processed {
        return Err(ParserError::BadConfig);
    }

    Ok(parser.file_info)
}


pub fn process_format_chunk(file_info: &mut FileInfo, bytes: &[u8]) -> Result<(), ParserError> {

    file_info.format = AudioFormat::from_code(util::u16_from_u8_slice(&bytes[0..1]));
    file_info.num_channels = util::u16_from_u8_slice(&bytes[2..3]);
    file_info.sample_rate = util::u32_from_u8_slice(&bytes[4..7]);

    // Unused chunk info
    // let byte_rate = util::u32_from_u8_slice(&bytes[8..11]);
    // let block_align = util::u32_from_u8_slice(&bytes[12..13]);
    
    file_info.bits_per_sample = util::u16_from_u8_slice(&bytes[14..15]);

    match file_info.format {
        AudioFormat::Invalid => return Err(ParserError::BadFormat),
        (_) => ()
    };

    Ok(())
}


pub fn get_sample_from_data(num_bits: u8, file_info: &FileInfo, data: u32) -> u32 {
    match (num_bits, &file_info.format, &file_info.bits_per_sample) {

        (8, AudioFormat::PCM, 8) => data::u8_from_pcm8(data as u8) as u32,
        (9, AudioFormat::PCM, 8) => data::u9_from_pcm8(data as u8) as u32,
        (10, AudioFormat::PCM, 8) => data::u10_from_pcm8(data as u8) as u32,
        (11, AudioFormat::PCM, 8) => data::u11_from_pcm8(data as u8) as u32,
        (12, AudioFormat::PCM, 8) => data::u12_from_pcm8(data as u8) as u32,

        (8, AudioFormat::PCM, 16) => data::u8_from_pcm16(data as u16) as u32,
        (9, AudioFormat::PCM, 16) => data::u9_from_pcm16(data as u16) as u32,

        (8, AudioFormat::PCM, 32) => data::u8_from_pcm32(data as u32) as u32,
        (9, AudioFormat::PCM, 32) => data::u9_from_pcm32(data as u32) as u32,

        (8, AudioFormat::IEEEFloat, 32) => data::u8_from_ieee(data as f32) as u32,
        (9, AudioFormat::IEEEFloat, 32) => data::u9_from_ieee(data as f32) as u32,

        (8, AudioFormat::EightBitALaw, 16) => data::u8_from_alaw(data as u8) as u32,
        (9, AudioFormat::EightBitALaw, 16) => data::u9_from_alaw(data as u8) as u32,
        
        (8, AudioFormat::EightBitMuLaw, 32) => data::u8_from_mulaw(data as u8) as u32,
        (9, AudioFormat::EightBitMuLaw, 32) => data::u9_from_mulaw(data as u8) as u32,

        (_, _, _) => 0
    }
}