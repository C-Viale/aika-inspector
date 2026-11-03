pub fn is_mp3(bytes: &[u8]) -> bool {
  let has_id3_header = bytes[0] == 0x49 // 'I'
  && bytes[1] == 0x44 // 'D'
  && bytes[2] == 0x33; // '3';

  if bytes.len() >= 10 && has_id3_header {
    let byte_6 = (bytes[6] as u32 & 0x7F) << 21;
    let byte_7 = (bytes[7] as u32 & 0x7F) << 14;
    let byte_8 = (bytes[8] as u32 & 0x7F) << 7;
    let byte_9 = bytes[9] as u32 & 0x7F;

    let size = byte_6 | byte_7 | byte_8 | byte_9;
    let mut offset = (10 + size) as usize;

    if (bytes[5] & 0x10) != 0 {
      offset += 10
    }

    return has_mp3_frames(&bytes[offset..]);
  }

  return has_mp3_frames(&bytes);
}

fn has_mp3_frames(bytes: &[u8]) -> bool {
  for i in 0..bytes.len() - 4 {
    if bytes[i] != 0xFF || (bytes[i + 1] & 0xE0) != 0xE0 {
      continue;
    }

    let version = (bytes[i + 1] >> 3) & 0x03;
    let layer = (bytes[i + 1] >> 1) & 0x03;
    let bitrate_index = (bytes[i + 2] >> 4) & 0x0F;
    let sample_rate_index = (bytes[i + 2] >> 2) & 0x03;

    if version == 1 || layer == 0 || bitrate_index == 0 || bitrate_index == 15 || sample_rate_index == 3 {
      continue;
    }

    return true;
  }

  false
}
