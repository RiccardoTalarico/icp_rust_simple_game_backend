# Simple Rust Game Backend

This Rust implementation serves as a very simple proof of concept for a game backend on the ICP

## Technologies Used

- **Rust**: The programming language used for the backend implementation.
- **Candid**: Utilized for defining the canister interface.
- **Internet Computer (IC)**: Employs IC-specific libraries (`ic_cdk`) for interactions with the Internet Computer.

## Components

### Data Structures

#### Player
Represents information about a player, including an identifier, username, level, experience, creation timestamp, and a vector of earned achievement IDs.

#### Achievement
Describes an achievement with fields for ID, name, description, and points (experience needed to get the achievement).

#### InventoryItem
Represents an item in the shared inventory (available to all players), with fields for ID, name, description, quantity (or earnable experience), and creation timestamp.

### Functionality

#### Player Management
- **Create Player**: Allows the creation of a new player with a specified username.
- **Get All Players**: Retrieves information about all players.
- **Get Player by ID**: Fetches details about a specific player using their identifier.
- **Update Player Level**: Modifies a player's level.

#### Achievement Management
- **Create Achievement**: Enables the creation of a new achievement with a specified name, description, and points.
- **Get All Achievements**: Retrieves information about all available achievements.
- **Get Achievement by ID**: Fetches details about a specific achievement using its identifier.
- **Earn Achievement**: Checks conditions and awards an achievement to a player.

#### Inventory Management
- **Create Inventory Item**: Allows the creation of a new item in the shared inventory with a specified name, description, and quantity (earnable experience).
- **Get All Inventory Items**: Retrieves information about all items in the shared inventory.
- **Get Inventory Item by ID**: Fetches details about a specific inventory item using its identifier.
- **Use Inventory Item**: Simulates using an inventory item, updating the player's experience and removing the used item.

### Error Handling

The system includes an `Error` enum with variants for handling various error scenarios, such as not finding entities, encountering invalid operations, or insufficient experience for certain actions.

## How to Use

1. **Create a Player**: Use the `create_player` function, providing a `PlayerPayload` with the desired `username`.
2. **Create Achievements**: Use the `create_achievement` function, providing an `AchievementPayload` with the achievement details.
3. **Create Items**: Use the `create_inventory_item` function, providing an `InventoryItemPayload` with the item details.
4. **Use an Inventory Item**: Use the `use_inventory_item` function, specifying the player's ID and the inventory item's ID (item quantity will be converted into player experience).
5. **Earn an Achievement**: Use the `earn_achievement` function, specifying the player's ID and the achievement's ID.

Feel free to explore, customize and add functionality based on your specific game logic and requirements. This project is intended as a basic proof of concept.


## Local Deployment

To deploy on a local machine, follow these steps:
1. **Clone the Repository**: Clone this repository to your local machine.

    ```bash
    git clone https://github.com/RiccardoTalarico/icp_rust_simple_game_backend
    ```

2. **Install Rust**: Ensure that Rust is installed on your machine. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).


3. **Install Dependencies**: Navigate to the project directory and install dependencies.

    ```bash
    cd icp_rust_simple_game_backend
    rustup target add wasm32-unknown-unknown
    cargo install candid-extractor
    DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
    echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
    ```


4. **Start the IC Replica Locally**: Run a replica of the internet computer locally using the following command.

```bash
# Starts the replica, running in the background
dfx start --background
```

4. **Deploy the Canister**:  .
```bash
# This runs "deploy": "./did.sh && dfx generate && dfx deploy -y"
npm  deploy
```

5. **Access the Endpoints**: Once the project is running, you can access the provided endpoints for interacting with the Canister.


If you have made changes to your backend canister, you can generate a new candid interface with

```bash
# This runs "generate": "./did.sh && dfx generate",

npm run generate
```
This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.
also to run dfx generate and dfx deploy simultaneously you can opt for running 

```bash
# This runs "gen-deploy":"./did.sh && dfx generate && dfx deploy -y"

npm run gen-deploy
```

if by any chance that you get error as :

note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest


you can fix it by following the instruction and adding the `resolver ="2"` to the workspace root's manifest it is in the file `Cargo.toml` eg:

        [workspace]
        members = [
            "src/icp_rust_simple_game_backend",
        ]
        resolver="2"

## Canister Deployment

To deploy this canister on the Internet Computer, you need to follow the steps outlined in the Internet Computer documentation. Here is a simplified overview:

1. **Internet Computer SDK**: Install the DFINITY Canister SDK by following the instructions at [https://sdk.dfinity.org/docs/download.html](https://sdk.dfinity.org/docs/download.html).

2. **Start the Canister**: Build the canister using the following command:

    ```bash
    dfx start
    ```

3. **Deploy the Canister**: Deploy the canister to the Internet Computer.

    ```bash
    dfx deploy
    ```

4. **Interact with the Canister**: Once deployed, you can interact with the canister using the generated canister ID.

    ```bash
    dfx canister call <canister-id> <function-name>
    ```