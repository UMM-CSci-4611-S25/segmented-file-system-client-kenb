# Segmented File System Client - Codebase Documentation

This document provides an overview of the key structures and their contents in the segmented file system client codebase. It also explains how these structures are intended to be used.

---

## **1. Packet Structure**

### **File**: packet.rs

The `Packet` enum represents the two types of packets used in the protocol: `Header` and `Data`.

#### **Enum: `Packet`**

```rust
pub enum Packet {
    Header(Header),
    Data(Data),
}
```

- **Variants**:
  - `Header`: Contains metadata about the file being transferred.
  - `Data`: Contains a chunk of the file's data.

---

#### **Struct: `Header`**

```rust
pub struct Header {
    pub file_id: u8,
    pub file_name: OsString,
}
```

- **Fields**:
  - `file_id` (`u8`): A unique identifier for the file.
  - `file_name` (`OsString`): The name of the file being transferred.

- **Usage**:
  - The `Header` packet is used to initialize a file transfer. It provides the file's name and associates it with a unique `file_id`.

---

#### **Struct: `Data`**

```rust
pub struct Data {
    pub file_id: u8,
    pub packet_number: u16,
    pub is_last_packet: bool,
    pub data: Vec<u8>,
}
```

- **Fields**:
  - `file_id` (`u8`): The unique identifier for the file this data belongs to.
  - `packet_number` (`u16`): The sequence number of this data packet.
  - `is_last_packet` (`bool`): Indicates whether this is the last packet for the file.
  - `data` (`Vec<u8>`): The actual data chunk.

- **Usage**:
  - The `Data` packet contains a chunk of the file's data. The `packet_number` helps order the chunks, and `is_last_packet` signals the end of the file.

---

#### **Packet Parsing**

The `Packet` enum implements `TryFrom<&[u8]>` to parse raw byte arrays into `Packet` objects.

```rust
impl TryFrom<&[u8]> for Packet {
    type Error = PacketParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // Placeholder implementation
        Err(PacketParseError::InvalidPacketFormat)
    }
}
```

- **Usage**:
  - Converts raw UDP data into `Packet` objects for further processing.

---

## **2. PacketGroup Structure**

### **File**: packet_group.rs

The `PacketGroup` struct manages all packets for a single file.

#### **Struct: `PacketGroup`**

```rust
pub struct PacketGroup {
    pub file_name: Option<OsString>,
    pub expected_packet_count: Option<usize>,
    pub packets: HashMap<u16, Vec<u8>>,
}
```

- **Fields**:
  - `file_name` (`Option<OsString>`): The name of the file (from the `Header` packet).
  - `expected_packet_count` (`Option<usize>`): The total number of packets expected for this file.
  - `packets` (`HashMap<u16, Vec<u8>>`): A map of packet numbers to their data.

- **Usage**:
  - Collects and organizes packets for a single file.
  - Ensures all packets are received and writes the file to disk.

---

#### **PacketGroup Methods**

```rust
impl PacketGroup {
    pub fn process_packet(&mut self, packet: Packet);
    pub fn received_all_packets(&self) -> bool;
    pub fn write_file(&self) -> io::Result<()>;
}
```

- **`process_packet`**:
  - Adds a `Packet` to the `PacketGroup`.
  - Handles both `Header` and `Data` packets.

- **`received_all_packets`**:
  - Checks if all expected packets have been received.

- **`write_file`**:
  - Writes the assembled file to disk.

---

## **3. FileManager Structure**

### **File**: file_manager.rs

The `FileManager` struct manages multiple `PacketGroup`s, one for each file being transferred.

#### **Struct: `FileManager`**

```rust
pub struct FileManager {
    files: HashMap<u8, PacketGroup>,
}
```

- **Fields**:
  - `files` (`HashMap<u8, PacketGroup>`): A map of `file_id` to `PacketGroup`.

- **Usage**:
  - Tracks all files being transferred.
  - Routes packets to the appropriate `PacketGroup`.

---

#### **FileManager Methods**

```rust
impl FileManager {
    pub fn received_all_packets(&self) -> bool;
    pub fn process_packet(&mut self, packet: Packet);
    pub fn write_all_files(&self) -> Result<(), std::io::Error>;
}
```

- **`received_all_packets`**:
  - Checks if all files have been completely received.

- **`process_packet`**:
  - Routes a `Packet` to the appropriate `PacketGroup`.

- **`write_all_files`**:
  - Writes all completed files to disk.

---

## **4. Error Handling**

### **File**: errors.rs

This file defines custom error types for the client.

#### **Enum: `PacketParseError`**

```rust
pub enum PacketParseError {
    TooShort,
    InvalidPacketFormat,
}
```

- **Variants**:
  - `TooShort`: Indicates that the packet is too short to be valid.
  - `InvalidPacketFormat`: Indicates that the packet format is invalid.

- **Usage**:
  - Used in the `TryFrom<&[u8]>` implementation for `Packet`.

---

#### **Enum: `ClientError`**

```rust
pub enum ClientError {
    IoError(std::io::Error),
    PacketParseError(PacketParseError),
}
```

- **Variants**:
  - `IoError`: Wraps an I/O error.
  - `PacketParseError`: Wraps a `PacketParseError`.

- **Usage**:
  - Represents errors that can occur during the client's operation.

---

## **5. Main Function**

### **File**: main.rs

The `main` function orchestrates the client’s operation.

#### **Workflow**

1. **Setup**:
   - Binds a UDP socket and connects to the server.
   - Initializes a `FileManager`.

2. **Receive Packets**:
   - Continuously receives packets from the server.
   - Parses each packet and processes it using the `FileManager`.

3. **Write Files**:
   - Once all packets are received, writes the files to disk.

#### **Code**

```rust
fn main() -> Result<(), ClientError> {
    let sock = UdpSocket::bind("0.0.0.0:7077")?;
    let remote_addr = "127.0.0.1:6014";
    sock.connect(remote_addr)?;
    let mut buf = [0; 1028];

    let mut file_manager = FileManager::default();

    while !file_manager.received_all_packets() {
        let len = sock.recv(&mut buf)?;
        let packet: Packet = buf[..len].try_into()?;
        file_manager.process_packet(packet);
    }

    file_manager.write_all_files()?;
    Ok(())
}
```

---

## **6. Testing**

### **File**: client_tests.sh

This script runs unit tests for the client. Focus on testing:

- `Packet` parsing.
- `PacketGroup` packet processing and file writing.
- `FileManager` packet routing and file management.
