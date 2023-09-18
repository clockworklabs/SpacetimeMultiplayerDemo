# SpacetimeMMO
Welcome to SpacetimeMMO, a basic game project developed in Unity that showcases the power and efficiency of SpacetimeDB in creating a massively multiplayer online game (MMO). While this project is still under development, all the code and assets are open-source and free for anyone to use in their own projects.

## Features

-   **Login**: The user's private key is stored in Unity PlayerPrefs system the first time they play and is used to authenticate in future sessions.
-  **Chat**: Open Chat window with ENTER key. Basic chat message system.
- **Overhead Names**: Player names are displayed over their head.
-   **Movement**: Basic player movement system.
-  **Resource Node Spawning**: Server spawns resource nodes on the map.
-  **Harvesting**: Players can harvest resource nodes.
-  **Inventory**: Open Inventory with TAB. Harvested resources go to inventory.
-  **Jumping**: Players can jump with SPACE.

## Getting Started

1. [Install SpacetimDB](https://spacetimedb.com/install) and start SpacetimeDB local instance. View the [SpacetimeDB getting started](https://spacetimedb.com/docs/getting-started) guide for detailed instructions.
2. Navigate to the Server folder. Publish the SpacetimeMMO module to your local instance.

`spacetime publish spacetimemmo`

3. Open the project in Unity. 
4. Open the Scene named `Main` in the Scenes folder.
5. Click on the `GameManager` object in the Scene and double check the connection settings in the Inspector. They should be:

`Module Address: spacetimemmo`

`Host Name: localhost:3000`

`SSL Enabled: false`

6. Hit the Play button to play in the editor.
