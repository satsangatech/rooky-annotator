Rooky Annotator is an open-source, interactive platform built to elevate your chess skills—whether you’re just learning the pieces or mastering advanced strategies. With dedicated modes for rookies and experts, it offers a smooth path from fundamentals to fluent gameplay. ♟️
---

## 🚀 Features

✅ **Decentralized & Secure:** Powered by Rust, Yew, and Nostr for speed, security, and resilience.
✅ **Interactive UI:** Styled with TailwindCSS for a sleek and modern interface.
✅ **Efficient Testing:** Ensured reliability with Rust’s built-in testing framework.

### 🧠 App Pages

The main page of the app includes two interactive tabs: **Rookie** and **Expert**.

- **Rookie Tab:** Presents a card displaying all the chess pieces. When a user selects a piece, a chessboard appears with that specific piece placed on it. The user can then click the piece to make valid moves, helping beginners understand how each piece operates.

- **Expert Tab:** Offers a keyboard interface for users to annotate chess moves directly, catering to experienced players who want to log and plan sequences.

---

## 🛠 Tech Stack

Rookye Annotator is built using the latest web technologies to ensure performance and scalability:

- **🦀 Rust** - High-performance, memory-safe programming language [(Download Rust)](https://www.rust-lang.org/learn/get-started)
- **🌿 Yew** - Rust-based framework for building interactive web apps [(Yew Docs)](https://yew.rs/docs/)
- **⚡ Nostr** - Decentralized protocol for secure communication [(Nostr Protocol)](https://nostr.com/)
- **🎨 TailwindCSS** - Utility-first CSS framework for styling [(Tailwind Docs)](https://tailwindcss.com/docs)
- **🗺️ Leaflet** - Interactive maps and location services [(LeafletJS)](https://leafletjs.com/)

---

## 🏗 Project Architecture

Rooky follows a modular frontend architecture within each app directory. The main source code resides in the `src/` folder, which is organized as follows:

- `src/` – Main application source code.
- `components/` – Reusable UI components (buttons, forms, widgets, etc.).
- `contexts/` – Application-wide state management and shared logic.
- `models/` – Users Nostr Metadata.
- `pages/` – Top-level views for the app.
- `Cargo.toml` – App-specific dependencies and metadata.
- `manifest.json` – Web app manifest for PWA support.
- `serviceWorker.js` – Enables offline functionality.
- `Trunk.toml` – Build configuration for Trunk.
- `build.rs` – Optional custom build scripts.

There is no centralized backend. All communication occurs via **Nostr** relays, and local data is stored in **IndexedDB**, ensuring a fully decentralized architecture.

---

## 🌐 How Rooky Uses Nostr

Fuente leverages **Nostr** as its primary communication protocol for a trustless and decentralized experience. 

### 🔗 Communication with Relays
- All messages, transactions, and interactions are sent via **Nostr relays**, removing the need for centralized servers.
- Each app (consumer, business, admin, etc.) subscribes to specific events in the network, ensuring seamless interaction.

### 🔑 Authentication
- Users authenticate using **Nostr public/private keys** instead of traditional logins.
- Private keys are securely stored on the client-side, never exposing them to third parties.

### 📦 Data Storage
- No centralized database is used. Instead, **IndexedDB** is leveraged for local storage.
- Users retain control over their data, which can be synchronized across devices via Nostr relays.

Using **Nostr** ensures Rooky remains **censorship-resistant**, **fault-tolerant**, and **decentralized**, making it an ideal solution for borderless commerce.

---

## 🔗 Useful Links

- 📜 Official Rust Docs: [Rust Documentation](https://doc.rust-lang.org/)
- 🔧 Trunk Guide: [Trunk Docs](https://trunkrs.dev/)
- 🖥️ Yew Framework: [Yew Documentation](https://yew.rs/docs/)
- 🔑 Nostr Protocol: [Nostr](https://nostr.com/)
- 🎨 TailwindCSS: [Tailwind Docs](https://tailwindcss.com/docs)