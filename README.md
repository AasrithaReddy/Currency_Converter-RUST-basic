# 💱 Currency Converter (Rust CLI)

A simple yet powerful command-line **Currency Converter** written in **Rust**, using real-time exchange rates from [exchangerate.host](https://exchangerate.host/). This project is perfect for learning Rust and working with public APIs, command-line interfaces, and asynchronous programming.

---

## 🚀 Features

- 🔁 **Multi-Currency Conversion**: Convert between any two international currencies.
- 🌐 **Real-Time Exchange Rates**: Live rates fetched via public API.
- 🧑‍💻 **User-Friendly CLI**: Clean and intuitive command-line interface using `clap`.

---

## 🧰 Built With

- 🦀 [Rust](https://www.rust-lang.org/)
- 📦 [`clap`](https://docs.rs/clap) for CLI parsing
- 🌐 [`reqwest`](https://docs.rs/reqwest) for HTTP requests
- 🔁 [`serde`](https://serde.rs/) for JSON parsing
- ⚡ [`tokio`](https://tokio.rs/) for async runtime

---

## 📦 Installation

Make sure you have Rust installed. If not, get it here: https://rustup.rs

```bash
git clone https://github.com/yourusername/currency_converter.git
cd currency_converter
cargo build --release
```

---

## 🧪 Usage

```bash
cargo run -- --from USD --to EUR --amount 100
```

### 📝 Example Output

```
100 USD = 92.36 EUR (Rate: 0.9236)
```

---

## 🧾 API Source

- [exchangerate.host](https://exchangerate.host) — free, no-auth exchange rate API.

---

## 📁 Project Structure

```
currency_converter/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

---

## 🙌 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you’d like to change.

---

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

## 💡 Author

**Your Name** – [@yourhandle](https://github.com/yourusername)
