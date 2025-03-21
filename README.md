# 🚀 Bitcoin Key Generator
A high-performance **Bitcoin private key brute-force generator** supporting multiple address types:
- **P2PKH (Legacy)**
- **P2WPKH (Bech32, SegWit)**

Designed for efficiency, using **multi-threading** and optimized **hash storage** to reduce RAM usage.

## 📌 **Features**
✅ **Generates Bitcoin private keys** and corresponding public addresses.

✅ **Supports multiple address types** (P2PKH, P2WPKH).

✅ **Multi-threaded execution** for high-speed key generation.

✅ **Efficient memory usage** (stores only 20-byte hashes instead of full addresses).

✅ **Fast hash lookup** using `rustc_hash::FxHashSet`.

## 🔧 **Installation**
### 1️⃣ **Clone the Repository**
```bash
git clone https://github.com/your-username/bitcoin-keygen.git
cd bitcoin-keygen
```

### 2️⃣ **Build the Project**
```bash
cargo build --release
```

### 3️⃣ **Run the Key Generator**
```bash
cargo run --release <THREADS>
```
- `<THREADS>` = Number of threads to use (e.g., `4`, `8`, `16`).

## ⚙️ **How It Works**
1. **Loads a database of Bitcoin addresses** from `data/bitcoin.tsv` (not included in the repository, can be found [here](https://privatekeyfinder.io/download/)).
2. **Generates random private keys** and derives public keys.
3. **Creates P2PKH and P2WPKH hashes** from the public key.
4. **Checks if the generated address exists** in the preloaded database.
5. **Prints the matching private key** if found.

## 📜 **Example Output**
```
[ INFO] Using 8 threads.
[ INFO] Loading hashes from a file...
[ INFO] Loaded 5,000,000 addresses (skipped 120).
[ INFO] Generating private keys...
[ INFO] Generated 5m keys.
[ INFO] Generated 10m keys.
[ INFO] Found the key: bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh - 5HueCGU8rMjxEXxiPuD5BDu6VPrmWx4NV9UtDzkcMRoKyjTbNY
```

## 🛠 **Technical Details**
- **Multi-threaded implementation** using `std::thread`.
- **Optimized hash comparison** using `FxHashSet<[u8; 20]>` (faster than `HashSet<String>`).
- **Bitcoin address encoding** via `bs58` (Base58) and `bech32`.
- **Private key generation** with `secp256k1::SecretKey`.
- **Hashing functions**: `SHA256`, `RIPEMD-160`.

## 🏆 **Planned Features**
🔹 **Ethereum and other chains address generation & brute-force search**.

🔹 **GPU acceleration for key generation**.

🔹 **Notification system (Telegram, Discord etc)**.

## ⚠️ **Legal Disclaimer**
This project is for **educational and research purposes only**.
**DO NOT use it for unauthorized access** or illegal activities.
The authors take no responsibility for any misuse.

## 📄 **License**  
This project is licensed under the **MIT License**.