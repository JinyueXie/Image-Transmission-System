# 🌐 Image Transmission System

The **Image Transmission System** is a project that demonstrates how to securely and reliably transmit image files between devices over a network using **C++** and **Rust**. This project focuses on real-world communication protocols (**TCP/UDP**) and includes modern security practices like encryption to ensure data confidentiality. By integrating multi-language development and advanced networking concepts, this system highlights key technical skills in **system integration** and **network programming**.

---

## 💡 Why This Project?

In real-world applications such as **aerospace systems**, **IoT devices**, or **mission-critical operations**, the ability to securely and efficiently transmit data across devices is essential. This project simulates a small-scale version of such systems:
- **Sender Device**: Represents an on-board system or sensor transmitting data.
- **Receiver Device**: Acts as a ground station or control system receiving and processing the data.

This project is designed to:
1. **Showcase secure and reliable communication** over TCP/UDP.
2. **Demonstrate multi-language integration** for performance and safety.
3. **Simulate real-world scenarios**, such as data transmission under network stress or unreliable conditions.

---

## 🛠️ How It Works

### System Overview
The system consists of two main components:
1. **Sender (C++):**  
   - Reads an image file from the local storage.
   - Encrypts the image using **AES encryption** for secure transmission.
   - Transmits the encrypted data over **TCP/UDP** to the receiver.

2. **Receiver (Rust):**  
   - Listens for incoming data on a specified port.
   - Decrypts the data using the same AES key.
   - Reconstructs and saves the image to local storage.

### Devices Used
This project can run on any combination of devices:
- **Sender**: Ubuntu machine or similar Linux-based environment.
- **Receiver**: Ubuntu machine.
- **Network Setup**: Use a Wi-Fi or local network to connect the devices.

---

## 🚀 Features

1. **Protocol Flexibility**:
   - **TCP**: Ensures reliable delivery with acknowledgment and retransmission.
   - **UDP**: Optimized for speed, useful for real-time or low-latency scenarios.

2. **Security**:
   - **AES-GCM Encryption**: Protects the image data during transmission, ensuring it cannot be intercepted or tampered with.

3. **Error Detection**:
   - Checksum-based validation ensures data integrity, especially for UDP transmissions.

4. **Device Compatibility**:
   - Supports cross-device communication between Ubuntu.

5. **Real-World Testing**:
   - Simulates real-world conditions such as network delays and packet loss using tools like `tc` (traffic control).

---

## 📂 Project Structure

```plaintext
Image-Transmission-System/
├── sender/         # C++ code for the sender
│   ├── sender.cpp  # Main program to send the image
│   ├── CMakeLists.txt # Build configuration
├── receiver/       # Rust code for the receiver
│   ├── src/        # Main Rust program files
│   ├── Cargo.toml  # Rust project configuration
├── images/         # Folder for test images
├── README.md       # Project documentation
└── LICENSE         # License file (optional)
