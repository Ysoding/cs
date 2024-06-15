// https://linux.die.net/man/5/pcap-savefile
// https://www.tcpdump.org/manpages/pcap-savefile.5.html

// Header:
//     - Magic Number (4 Bytes)
//     - Minor Version Number (2 Bytes)
//     - Minor Version Number (2 Bytes)
//     - Time Zone Offset (4 Bytes) - always zero
//     - Accuracy of time stamps (4 Bytes) - always zero
//     - Snapshot length (4 Bytes) - i.e. truncation threshold
//     - Link-Layer header type (4 Bytes) - see pcap-linktype

// Following this are zero or more packets

// Packet Headers:
//     - Time stamp of packet capture (4 bytes)
//     - Time since capture in micro or nano seconds (4 bytes)
//     - Number of bytes of captured data that follow the per-packet header (4 bytes)
//     - Number of bytes that would have been captured without truncation (4 bytes)

// All fields in byte order of the host writing the file (little endian)
//
use std::io::{self, Read};

fn main() {
    // cd syn_flood/ && cargo build && cd .. && cat synflood.pcap | syn_flood/target/debug/syn_flood
    // 95829 packets parsed with 56298 connections, 39531 (70.22%) acknowledged
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .expect("Failed to read from stdin");

    let f = pcap::File::from_bytes(&buffer).unwrap();

    let mut initiated: f32 = 0.;
    let mut acknowledged: f32 = 0.;
    for packet in f.packets.iter() {
        // link layer header (4 bytes)
        // network layer header
        // transport layer header
        // application layer data
        let ipv4_packet = ipv4::Packet::from_bytes(&packet.payload[4..]).unwrap();
        let tcp_header = tcp::SegmentHeader::from_bytes(&ipv4_packet.payload).unwrap();
        if tcp_header.is_initiated() {
            initiated += 1.;
        }
        if tcp_header.is_acknowledgment() {
            acknowledged += 1.;
        }
    }

    println!(
        "{} packets parsed with {} connections, {} ({:.2}%) acknowledged",
        f.packets.len(),
        initiated,
        acknowledged,
        acknowledged / initiated * 100.
    )
}

mod pcap {
    use packet::*;
    use std::io::{self, Cursor};

    use byteorder::{LittleEndian, ReadBytesExt};

    #[derive(Debug)]
    pub struct File {
        pub header: Header,
        pub packets: Vec<Packet>,
    }

    impl File {
        pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
            let header = Header::from_bytes(&data[0..24])?;
            let mut packets = Vec::new();

            let mut offset = 24;
            while offset < data.len() {
                let packet = Packet::from_bytes(&data[offset..])?;
                offset += 16 + packet.header.n_bytes_captured as usize;
                packets.push(packet);
            }

            Ok(Self { header, packets })
        }
    }

    #[derive(Debug)]
    struct Header {
        raw: Vec<u8>,
        magic_number: u32,
        major_version: u16,
        minor_version: u16,
        time_zone_offset: i32,
        time_step_accuracy: u32,
        snapshot_length: u32,
        link_layer_header_type: LinkLayerHeaderType,
    }

    impl Header {
        fn from_bytes(data: &[u8]) -> io::Result<Self> {
            let mut cursor = Cursor::new(data);

            let magic_number = cursor.read_u32::<LittleEndian>()?;
            let major_version = cursor.read_u16::<LittleEndian>()?;
            let minor_version = cursor.read_u16::<LittleEndian>()?;
            let time_zone_offset = cursor.read_i32::<LittleEndian>()?;
            let time_step_accuracy = cursor.read_u32::<LittleEndian>()?;

            assert!(magic_number == 0xa1b2c3d4);
            assert!(major_version == 2);
            assert!(minor_version == 4);
            assert!(time_zone_offset == 0);
            assert!(time_step_accuracy == 0);

            let snapshot_length = cursor.read_u32::<LittleEndian>()?;
            let link_layer_header_type = match cursor.read_u32::<LittleEndian>()? {
                0 => LinkLayerHeaderType::Null,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unknown link layer header type",
                    ))
                }
            };

            Ok(Self {
                raw: data.to_vec(),
                magic_number,
                major_version,
                minor_version,
                time_zone_offset,
                time_step_accuracy,
                snapshot_length,
                link_layer_header_type,
            })
        }
    }

    #[derive(Debug)]
    #[repr(u32)]
    pub enum LinkLayerHeaderType {
        Null = 0,
    }

    mod packet {
        use std::io::{self, Cursor};

        use byteorder::{LittleEndian, ReadBytesExt};

        #[derive(Debug)]
        pub struct Header {
            pub raw: Vec<u8>,
            pub capture_time_stamp: u32,
            pub time_since_capture: u32,
            pub n_bytes_captured: u32,
            pub n_bytes_captured_wo_truncation: u32,
        }

        impl Header {
            pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
                let mut cursor = Cursor::new(data);

                Ok(Self {
                    raw: data.to_vec(),
                    capture_time_stamp: cursor.read_u32::<LittleEndian>()?,
                    time_since_capture: cursor.read_u32::<LittleEndian>()?,
                    n_bytes_captured: cursor.read_u32::<LittleEndian>()?,
                    n_bytes_captured_wo_truncation: cursor.read_u32::<LittleEndian>()?,
                })
            }
        }

        #[derive(Debug)]
        pub struct Packet {
            pub header: Header,
            pub payload: Vec<u8>,
        }

        impl Packet {
            pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
                let header = Header::from_bytes(&data[0..16])?;
                let payload = data[16..header.n_bytes_captured as usize + 16].to_vec();
                Ok(Self { header, payload })
            }
        }
    }
}

mod ipv4 {
    use std::io;

    pub struct Packet {
        pub header: Header,
        pub payload: Vec<u8>,
    }

    impl Packet {
        pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
            let header = Header::from_bytes(&data[0..20])?;
            let payload = data[20..header.total_length as usize].into();
            Ok(Self { header, payload })
        }
    }

    pub struct Header {
        pub raw: Vec<u8>,
        pub version: u8,
        pub ihl: u8,
        pub dscp: u8,
        pub ecn: u8,
        pub total_length: u16,
        pub identification: u16,
        pub flags: u8,
        pub fragment_offset: u16,
        pub time_to_live: u8,
        pub protocol: u8,
        pub header_checksum: u16,
        pub source_ip_address: u32,
        pub destination_ip_address: u32,
    }

    impl Header {
        pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
            // big-edian (network byte order)
            let version = (data[0] >> 4) & 0xF;
            let ihl = data[0] & 0x0F;

            let dscp = (data[1] >> 2) & 0x3F;
            let ecn = data[1] & 0x03;

            let total_length = u16::from_be_bytes([data[2], data[3]]);

            let identification = u16::from_be_bytes([data[4], data[5]]);

            let flags_fragment_offset = u16::from_be_bytes([data[6], data[7]]);
            let flags = ((flags_fragment_offset >> 13) & 0x07) as u8;
            let fragment_offset = (flags_fragment_offset & 0x1FFF) as u16;

            let time_to_live = data[8];
            let protocol = data[9];

            let header_checksum = u16::from_be_bytes([data[10], data[11]]);
            let source_ip_address = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);
            let destination_ip_address =
                u32::from_be_bytes([data[16], data[17], data[18], data[19]]);

            let raw = data.to_vec();
            return Ok(Self {
                raw,
                version,
                ihl,
                dscp,
                ecn,
                total_length,
                identification,
                flags,
                fragment_offset,
                time_to_live,
                protocol,
                header_checksum,
                source_ip_address,
                destination_ip_address,
            });
        }
    }
}

mod tcp {
    #[derive(Debug)]
    pub struct SegmentHeader {
        pub source_port: u16,
        pub destination_port: u16,
        pub sequence_number: u32,
        pub acknowledgment_number: u32,
        pub data_offset: u8,
        pub flags: TcpFlags,
        pub window_size: u16,
        pub checksum: u16,
        pub urgent_pointer: u16,
        pub options: Vec<u8>,
        pub payload: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct TcpFlags {
        pub cwr: bool, // Congestion Window Reduced (CWR) flag
        pub ece: bool, // ECN-Echo flag
        pub urg: bool, // Urgent pointer field significant
        pub ack: bool, // Acknowledgment field significant
        pub psh: bool, // Push Function
        pub rst: bool, // Reset the connection
        pub syn: bool, // Synchronize sequence numbers
        pub fin: bool, // No more data from sender
    }

    impl SegmentHeader {
        pub fn from_bytes(data: &[u8]) -> Option<Self> {
            if data.len() < 20 {
                return None; // Minimum TCP header size is 20 bytes
            }

            let source_port = u16::from_be_bytes([data[0], data[1]]);
            let destination_port = u16::from_be_bytes([data[2], data[3]]);

            let sequence_number = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);

            let acknowledgment_number = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);

            let data_offset = (data[12] >> 4) * 4; // Data offset in 4-byte words
            let flags = TcpFlags {
                cwr: (data[13] & 0b1000_0000) != 0,
                ece: (data[13] & 0b0100_0000) != 0,
                urg: (data[13] & 0b0010_0000) != 0,
                ack: (data[13] & 0b0001_0000) != 0,
                psh: (data[13] & 0b0000_1000) != 0,
                rst: (data[13] & 0b0000_0100) != 0,
                syn: (data[13] & 0b0000_0010) != 0,
                fin: (data[13] & 0b0000_0001) != 0,
            };
            let window_size = u16::from_be_bytes([data[14], data[15]]);

            let checksum = u16::from_be_bytes([data[16], data[17]]);
            let urgent_pointer = u16::from_be_bytes([data[18], data[19]]);

            let options = data[20..(data_offset as usize)].to_vec(); // Options field
            let payload = data[data_offset as usize..].to_vec(); // TCP payload

            Some(SegmentHeader {
                source_port,
                destination_port,
                sequence_number,
                acknowledgment_number,
                data_offset,
                flags,
                window_size,
                checksum,
                urgent_pointer,
                options,
                payload,
            })
        }

        pub fn is_acknowledgment(&self) -> bool {
            self.flags.ack && self.is_client_packet()
        }

        pub fn is_initiated(&self) -> bool {
            self.flags.syn && self.is_server_packet()
        }

        pub fn is_client_packet(&self) -> bool {
            self.source_port == 80
        }
        pub fn is_server_packet(&self) -> bool {
            self.destination_port == 80
        }
    }
}
