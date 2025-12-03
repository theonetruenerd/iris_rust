
pub struct NmeaBuffer {
    buffer: [u8; 512],
    write_pos: usize,
    read_pos: usize,
}

impl NmeaBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0u8; 512],
            write_pos: 0,
            read_pos: 0,
        }
    }

    /// Add new data to the buffer
    pub fn add_data(&mut self, data: &[u8]) {
        for &byte in data {
            self.buffer[self.write_pos] = byte;
            self.write_pos = (self.write_pos + 1) % self.buffer.len();

            // Prevent overflow - this is a safety measure
            if self.write_pos == self.read_pos {
                self.read_pos = (self.read_pos + 1) % self.buffer.len();
            }
        }
    }

    /// Try to extract a complete NMEA sentence (ending with \r\n)
    pub fn get_sentence(&mut self) -> Option<NmeaSentence> {
        let mut sentence_len = 0;
        let mut pos = self.read_pos;

        // Search for \r\n
        while pos != self.write_pos {
            if sentence_len > 0 && self.buffer[pos] == b'\n' &&
                self.buffer[(pos + self.buffer.len() - 1) % self.buffer.len()] == b'\r' {
                // Found end of sentence
                sentence_len += 1;
                break;
            }
            pos = (pos + 1) % self.buffer.len();
            sentence_len += 1;
        }

        // If we found a complete sentence
        if pos != self.write_pos && self.buffer[pos] == b'\n' {
            let mut sentence_data = [0u8; 128];
            let mut idx = 0;
            let mut temp_pos = self.read_pos;

            while temp_pos != pos {
                sentence_data[idx] = self.buffer[temp_pos];
                idx += 1;
                temp_pos = (temp_pos + 1) % self.buffer.len();
            }

            // Skip the \r\n
            self.read_pos = (pos + 1) % self.buffer.len();

            return Some(NmeaSentence {
                data: sentence_data,
                length: idx,
            });
        }

        None
    }
}

pub struct NmeaSentence {
    pub data: [u8; 128],
    pub length: usize,
}

impl NmeaSentence {
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.data[..self.length.saturating_sub(2)]) // Remove \r\n
    }
}