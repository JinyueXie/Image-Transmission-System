#include <boost/asio.hpp>
#include <fstream>
#include <iostream>
#include <vector>
#include <thread>
#include <chrono>
#include <openssl/evp.h>
#include <openssl/rand.h>
#include <cstring>

using namespace boost::asio;

// Function to encrypt data using OpenSSL EVP API
std::vector<unsigned char> encrypt_data(const std::vector<unsigned char>& data, const unsigned char* key) {
    EVP_CIPHER_CTX* ctx = EVP_CIPHER_CTX_new();
    if (!ctx) {
        throw std::runtime_error("Failed to create cipher context");
    }

    // Initialize encryption operation
    if (EVP_EncryptInit_ex(ctx, EVP_aes_128_ecb(), nullptr, key, nullptr) != 1) {
        EVP_CIPHER_CTX_free(ctx);
        throw std::runtime_error("Failed to initialize encryption");
    }

    // Prepare output buffer (includes space for padding)
    std::vector<unsigned char> encrypted_data(data.size() + EVP_MAX_BLOCK_LENGTH);
    int out_len1 = 0;
    int out_len2 = 0;

    // Encrypt data
    if (EVP_EncryptUpdate(ctx, encrypted_data.data(), &out_len1,
                         data.data(), data.size()) != 1) {
        EVP_CIPHER_CTX_free(ctx);
        throw std::runtime_error("Failed to encrypt data");
    }

    // Finalize encryption
    if (EVP_EncryptFinal_ex(ctx, encrypted_data.data() + out_len1, &out_len2) != 1) {
        EVP_CIPHER_CTX_free(ctx);
        throw std::runtime_error("Failed to finalize encryption");
    }

    // Cleanup
    EVP_CIPHER_CTX_free(ctx);

    // Resize to actual encrypted data size
    encrypted_data.resize(out_len1 + out_len2);
    return encrypted_data;
}

void send_image(const std::string& filename, const std::string& host, int port, bool use_tcp) {
    try {
        io_context io_context;
        std::ifstream file(filename, std::ios::binary);

        if (!file) {
            throw std::runtime_error("Error: Could not open file " + filename);
        }

        // Read file into vector
        std::vector<unsigned char> data(
            (std::istreambuf_iterator<char>(file)),
            std::istreambuf_iterator<char>()
        );

        // Define AES key (16 bytes for AES-128)
        const unsigned char aes_key[16] = {
            0x2D, 0x52, 0xA8, 0xF6, 0xA9, 0x25, 0x19, 0xFA,
            0x80, 0xF2, 0xFE, 0xAF, 0x09, 0x9B, 0xBF, 0x01
        };

        // Encrypt the data
        auto encrypted_data = encrypt_data(data, aes_key);
        std::cout << "Data encrypted, size: " << encrypted_data.size() << " bytes\n";

        const size_t CHUNK_SIZE = 1024; // 1KB chunks for UDP
        
        if (use_tcp) {
            // TCP implementation
            ip::tcp::socket socket(io_context);
            socket.connect(ip::tcp::endpoint(ip::address::from_string(host), port));

            // Send data size first
            uint64_t size = encrypted_data.size();
            write(socket, buffer(&size, sizeof(size)));

            // Send data in chunks
            size_t offset = 0;
            while (offset < encrypted_data.size()) {
                size_t chunk_size = std::min(CHUNK_SIZE, encrypted_data.size() - offset);
                write(socket, buffer(encrypted_data.data() + offset, chunk_size));
                offset += chunk_size;
                std::cout << "Sent " << offset << " of " << encrypted_data.size() << " bytes\n";
            }
        } else {
            // UDP implementation
            ip::udp::socket socket(io_context);
            ip::udp::endpoint endpoint(ip::address::from_string(host), port);
            socket.open(ip::udp::v4());

            // Send total size first
            uint64_t size = encrypted_data.size();
            socket.send_to(buffer(&size, sizeof(size)), endpoint);
            
            // Small delay to ensure size is received
            std::this_thread::sleep_for(std::chrono::milliseconds(100));

            // Send data in chunks
            size_t offset = 0;
            size_t chunk_number = 0;
            while (offset < encrypted_data.size()) {
                size_t chunk_size = std::min(CHUNK_SIZE, encrypted_data.size() - offset);
                
                // Prepare chunk header (8 bytes for chunk number)
                std::vector<unsigned char> chunk_data(chunk_size + sizeof(chunk_number));
                std::memcpy(chunk_data.data(), &chunk_number, sizeof(chunk_number));
                std::memcpy(chunk_data.data() + sizeof(chunk_number), 
                          encrypted_data.data() + offset, 
                          chunk_size);

                socket.send_to(buffer(chunk_data), endpoint);
                
                offset += chunk_size;
                chunk_number++;
                
                std::cout << "Sent chunk " << chunk_number << ", offset: " 
                          << offset << " of " << encrypted_data.size() << " bytes\n";
                
                // Small delay between chunks to prevent overwhelming the receiver
                std::this_thread::sleep_for(std::chrono::milliseconds(1));
            }

            // Send end marker
            uint64_t end_marker = UINT64_MAX;
            socket.send_to(buffer(&end_marker, sizeof(end_marker)), endpoint);
        }

        std::cout << "Image sent successfully!\n";
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
    }
}

int main(int argc, char* argv[]) {
    if (argc < 5) {
        std::cerr << "Usage: " << argv[0] << " <filename> <host> <port> <tcp|udp>\n";
        return 1;
    }

    send_image(argv[1], argv[2], std::stoi(argv[3]), std::string(argv[4]) == "tcp");
    return 0;
}
