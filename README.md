# Minecraft Server Interface
Control your locally run Minecraft server with ease using a GUI.

#### THIS PROJECT IS STILL IN DEVELOPMENT. THERE IS NO GUARANTEE THAT IT WILL WORK PROPERLY OR AT ALL.

### Current features:
- Download the latest [Paper](https://papermc.io/) server
- Launch the server
- Restart the server
- Server console preview
- Run commands in the server console

### Planned features:
- File browser
- More download options (i.e. [Fabric](https://fabricmc.net/), Vanilla, etc.)
- Change server properties
- Managing players (OP, ban, etc.)
- Managing worlds
- Managing plugins/mods
- Performance graphs (RAM, CPU, TPS)
- Download plugins/mods from Modrinth

### Dependencies:
- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/)

### How to use:
1. Make sure you have [Node.js](https://nodejs.org/en/) and [Rust](https://www.rust-lang.org/) installed.
2. Clone this repository:
    ```
    git clone https://github.com/TennoAntenno/mc-server-interface.git
    ```
3. Change the directory to the cloned repository:
    ```
    cd mc-server-interface
    ```
4. Run the following command to initialize Node:
    ```
    npm init
    ```
5. Run the project:
    ```
    npm run tauri dev
    ```