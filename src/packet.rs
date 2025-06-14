use std::collections::HashMap;
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct FirstPacket {
    pub packet_len: u32,
    pub packet_id: u32,
    pub version: u32,
    pub domain: String,
    pub port: u16,
    pub state: u32,
}

pub async fn read_varint(stream: &mut TcpStream) -> io::Result<u32> {
    let mut num_read = 0;
    let mut result = 0u32;
    loop {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).await?;
        let byte = buf[0];
        result |= ((byte & 0x7F) as u32) << (7 * num_read);
        num_read += 1;
        if byte & 0x80 == 0 {
            break;
        }
        if num_read > 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt overflow",
            ));
        }
    }
    Ok(result)
}

pub async fn read_string(stream: &mut TcpStream) -> io::Result<String> {
    let len = read_varint(stream).await?;
    let mut buf = vec![0; len as usize];
    stream.read_exact(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

pub async fn parse_first_packet(stream: &mut TcpStream) -> io::Result<(FirstPacket, Vec<u8>)> {
    let packet_len = read_varint(stream).await?;

    let packet_len_varint = encode_varint(packet_len as i32);
    let mut buf = vec![0u8; packet_len as usize];
    let _ = stream.peek(&mut buf).await?;
    let mut full_packet_buff = Vec::with_capacity(packet_len_varint.len() + buf.len());
    full_packet_buff.extend_from_slice(&packet_len_varint);
    full_packet_buff.extend_from_slice(&buf);

    let packet_id = read_varint(stream).await?;
    let version = read_varint(stream).await?;
    let domain = read_string(stream).await?;
    let mut port_buf = [0u8; 2];
    stream.read_exact(&mut port_buf).await?;
    let port = u16::from_be_bytes(port_buf);
    let state = read_varint(stream).await?;

    Ok((FirstPacket {
        packet_len,
        packet_id,
        version,
        domain,
        port,
        state,
    }, full_packet_buff))
}

pub fn encode_varint(value: i32) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut uvalue = value as u32;  // 符号なしに変換して扱う

    loop {
        let mut temp = (uvalue & 0b0111_1111) as u8;
        uvalue >>= 7;  // 符号なし右シフトとして機能する
        if uvalue != 0 {
            temp |= 0b1000_0000;  // 続きビットをセット
        }
        buffer.push(temp);
        if uvalue == 0 {
            break;
        }
    }
    buffer
}

pub fn create_login_disconnect_packet(message: &str) -> Vec<u8> {
    let json_component = format!(r#"{{"text":"{}"}}"#, message);
    let json_bytes = json_component.as_bytes();
    let json_length = encode_varint(json_bytes.len() as i32);
    let packet_id = encode_varint(0x00);

    let mut packet_data = Vec::new();
    packet_data.extend(packet_id);
    packet_data.extend(json_length);
    packet_data.extend(json_bytes);

    let packet_length = encode_varint(packet_data.len() as i32);

    let mut full_packet = Vec::new();
    full_packet.extend(packet_length);
    full_packet.extend(packet_data);

    full_packet
}

pub fn generate_unknown_domain_message(
    domain: &str,
    domain_map: &HashMap<String, String>,
) -> String {
    let mut keys: Vec<&String> = domain_map.keys().collect();
    keys.sort();

    let keys_str = domain_map
        .keys()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    format!(
        "§l{}§r は見つかりませんでした。\n以下のサーバーリストから再度ご選択ください。\n\n{}\n",
        domain, keys_str
    )
}
