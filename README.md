# BitCraftMini

BitCraftMini is a very small feature set version of our game BitCraft. Right now you can walk around, chop down trees, and trade with other players.

## 1. Download

You can git-clone BitCraftMini from here:

```
git clone ssh://git@github.com/clockworklabs/BitCraftMini
```

Once you have downloaded BitCraftMini, you will need to compile the spacetime module.

## 2. Compile the Spacetime Module

In order to compile the BitCraftMini module, you will need to install cargo. You can install cargo from here:

 > https://www.rust-lang.org/tools/install

Once you have cargo installed, you can compile and publish the module with these commands:

```bash
cd BitCraftMini/Server
spacetime publish --clear-database
```

`spacetime publish` will output an address where your module has been deployed to. You will want to copy/save this address because you will need it in step 3. Here is an example of what it should look like:

```
$ spacetime publish --clear-database
info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
    Finished release [optimized] target(s) in 0.03s
Publish finished successfully.
Created new database with address: c91c17ecdcea8a05302be2bad9dd59b3
```

Once you have published your module, you can generate the client files required to connect to your module:

```bash
spacetime generate --out-dir ../Client/Assets/_Project/autogen --lang=cs
```

Here is some sample output:

```
$ spacetime generate --out-dir ../Client/Assets/_Project/autogen --lang cs
info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
    Finished release [optimized] target(s) in 0.03s
compilation took 234.613518ms
Generate finished successfully.
```

If you've gotten this message then everything should be working properly so far.

## 3. Replace address in BitCraftMiniGameManager

Now that you have published the module to a new address, we will need to put this address in `BitCraftMiniGameManager.cs`.

Inside of BitCraftMiniGameManager.cs, you should see a `Start()` function similar to this:

```cs
protected void Start()
{
    FeatureRoot = new GameObject("Features");

    Application.targetFrameRate = 60;

    NetworkManager.instance.onConnect += () => { Debug.Log("Connected."); };
    NetworkManager.instance.onConnectError += a =>
    {
        Debug.LogError($"Connection error: " + (a.HasValue ? a.Value.ToString() : "Null"));
    };
    NetworkManager.instance.onDisconnect += (closeStatus, error) =>
    {
        Debug.Log("Disconnected.");
    };

    NetworkManager.instance.onRowUpdate += OnRowUpdate;
    NetworkManager.instance.onEvent += OnEvent;

    NetworkManager.instance.onIdentityReceived += (identity) => {
        NetworkPlayer.identity = identity;
    };

    NetworkManager.instance.onTransactionComplete += CheckNewPlayer;

    NetworkManager.instance.Connect("spacetimedb.com/spacetimedb", "bitcraftmini");
}
```

The last line is the one we want to change. We want to take our address and change from this:

```cs
	NetworkManager.instance.Connect("spacetimedb.com/spacetimedb", "bitcraftmini");
```

to this:

```cs
	NetworkManager.instance.Connect("spacetimedb.com/spacetimedb", "c91c17ecdcea8a05302be2bad9dd59b3");
```

Instead of putting `c91c17ecdcea8a05302be2bad9dd59b3`, you should use your own address that you generated in step 2.

## 4. Play Mode

You should now be able to enter play mode and walk around! You can mine some rocks, cut down some trees and if you connect more clients you can trade with other players.


## 5. Editing the Module

If you want to make further updates to the module, make sure to use this publish command instead:

```bash
spacetime publish c91c17ecdcea8a05302be2bad9dd59b3 --clear-database
``` 

Where `c91c17ecdcea8a05302be2bad9dd59b3` is your own address. If you do this instead then you won't have to change the address inside of `BitCraftMiniGameManager.cs`

When you change the server module you should also regenerate the client files as well:

```bash
spacetime generate --out-dir ../Client/Assets/_Project/autogen --lang=cs
```

You may want to consider putting these 2 commands into a simple shell script to make the process a bit cleaner.
