use std::io::Write;

use base64::{engine::general_purpose::STANDARD, Engine};
use flate2::{write::ZlibEncoder, Compression};

const CHUNK_SIZE: usize = 4096;

pub struct Placement {
    pub image_id: u32,
    pub placement_id: u32,
    pub cols: u16,
    pub rows: u16,
}

pub fn transmit_and_display(
    rgb: &[u8],
    width: usize,
    height: usize,
    placement: &Placement,
) -> Vec<u8> {
    assert_eq!(
        rgb.len(),
        width * height * 3,
        "rgb buffer does not match image size"
    );

    let keys = format!(
        "a=T,f=24,o=z,s={width},v={height},c={},r={},i={},p={},C=1,q=2",
        placement.cols, placement.rows, placement.image_id, placement.placement_id
    );
    command(&keys, &compress(rgb))
}

pub fn delete_all() -> Vec<u8> {
    command("a=d,d=A,q=2", &[])
}

fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::with_capacity(data.len() / 4), Compression::fast());
    encoder
        .write_all(data)
        .expect("writing to a Vec cannot fail");
    encoder.finish().expect("writing to a Vec cannot fail")
}

fn command(keys: &str, payload: &[u8]) -> Vec<u8> {
    let encoded = STANDARD.encode(payload);
    if encoded.is_empty() {
        return format!("\x1b_G{keys}\x1b\\").into_bytes();
    }

    let chunks: Vec<&[u8]> = encoded.as_bytes().chunks(CHUNK_SIZE).collect();
    let mut out = Vec::with_capacity(encoded.len() + chunks.len() * 16 + keys.len());
    for (i, chunk) in chunks.iter().enumerate() {
        let more = u8::from(i + 1 < chunks.len());
        out.extend_from_slice(b"\x1b_G");
        if i == 0 {
            out.extend_from_slice(keys.as_bytes());
            out.push(b',');
        }
        write!(out, "m={more};").expect("writing to a Vec cannot fail");
        out.extend_from_slice(chunk);
        out.extend_from_slice(b"\x1b\\");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn commands(bytes: &[u8]) -> Vec<String> {
        String::from_utf8(bytes.to_vec())
            .unwrap()
            .split_terminator("\x1b\\")
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn command_without_payload() {
        assert_eq!(commands(&delete_all()), ["\x1b_Ga=d,d=A,q=2"]);
    }

    #[test_case(1 => 1; "tiny")]
    #[test_case(3072 => 1; "exactly one chunk")] // 3072 bytes encode to 4096 base64 chars
    #[test_case(3073 => 2; "one byte over")]
    #[test_case(3072 * 3 => 3; "three chunks")]
    fn chunk_count(len: usize) -> usize {
        commands(&command("a=T", &vec![0; len])).len()
    }

    #[test]
    fn only_first_chunk_has_keys_and_last_ends_with_m0() {
        let cmds = commands(&command("a=T,i=1", &vec![7; 3072 * 3]));
        assert!(cmds[0].starts_with("\x1b_Ga=T,i=1,m=1;"));
        assert!(cmds[1].starts_with("\x1b_Gm=1;"));
        assert!(cmds[2].starts_with("\x1b_Gm=0;"));
    }

    #[test]
    fn payload_roundtrips() {
        let rgb: Vec<u8> = (0..2 * 3 * 3).map(|i| i as u8).collect();
        let placement = Placement {
            image_id: 1,
            placement_id: 1,
            cols: 1,
            rows: 1,
        };
        let cmds = commands(&transmit_and_display(&rgb, 2, 3, &placement));
        let payload: String = cmds.iter().map(|c| c.split_once(';').unwrap().1).collect();

        let compressed = STANDARD.decode(payload).unwrap();
        let mut decoded = Vec::new();
        std::io::Read::read_to_end(
            &mut flate2::read::ZlibDecoder::new(&compressed[..]),
            &mut decoded,
        )
        .unwrap();
        assert_eq!(decoded, rgb);
    }
}
