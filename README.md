# Image Transmission System

A project designed to demonstrate secure, reliable, and real-time image transmission using **C++** and **Rust**. The system transmits encrypted image data over TCP/UDP protocols, ensuring data confidentiality and integrity.

---

## 📌 Features
- **Protocol Support**:  
  - TCP: Reliable, connection-oriented communication.  
  - UDP: Fast, connectionless communication.  
- **Data Encryption**:  
  - AES encryption to secure transmitted data.  
- **Error Detection**:  
  - Checksum-based validation to ensure data integrity.  
- **Multi-Language Integration**:  
  - C++ for the sender, Rust for the receiver.  
- **Simulation-Driven Testing**:  
  - Supports integration with simulation tools like X-Plane or Unreal Engine.

---

## 📂 Directory Structure
```plaintext
Image-Transmission-System/
├── sender/         # C++ code for the sender
├── receiver/       # Rust code for the receiver
├── images/         # Test image files
├── README.md       # Project documentation
└── LICENSE         # Optional license file

