using SpacetimeDB;
using System.Linq;
using UnityEngine;
using ClientApi;

public partial class BitCraftMiniGameManager
{
    // Start is called before the first frame update
    void RegisterHandlers()
    {
        Reducer.OnCreateNewPlayerEvent += OnCreateNewPlayerEvent;

        NpcComponent.OnInsert += OnNpcComponentInsert;
        NpcComponent.OnUpdate += OnNpcComponentUpdate;
        NpcComponent.OnDelete += OnNpcComponentDelete;

        PlayerComponent.OnInsert += OnPlayerComponentInsert;
        PlayerComponent.OnUpdate += OnPlayerComponentUpdate;

        TransformComponent.OnInsert += OnTransformComponentInsert;
        TransformComponent.OnUpdate += OnTransformComponentUpdate;

        AnimationComponent.OnInsert += OnAnimationComponentInsert;
        AnimationComponent.OnUpdate += OnAnimationComponentUpdate;

        InventoryComponent.OnInsert += OnInventoryComponentInsert;
        InventoryComponent.OnUpdate += OnInventoryComponentUpdate;

        PlayerLoginComponent.OnInsert += OnPlayerLoginComponentInsert;
        PlayerLoginComponent.OnUpdate += OnPlayerLoginComponentUpdate;

        PlayerChatMessage.OnInsert += OnPlayerChatMessageInsert;
        PlayerChatMessage.OnUpdate += OnPlayerChatMessageUpdate;

        Chunk.OnInsert += OnChunkInsert;
        Chunk.OnUpdate += OnChunkUpdate;

        ResourceComponent.OnInsert += OnResourceComponentInsert;
        ResourceComponent.OnUpdate += OnResourceComponentUpdate;
        ResourceComponent.OnDelete += OnResourceComponentDelete;

        TradeSessionComponent.OnInsert += OnTradeSessionComponentInsert;
        TradeSessionComponent.OnUpdate += OnTradeSessionComponentUpdate;
        TradeSessionComponent.OnDelete += OnTradeSessionComponentDelete;

    }

    void OnCreateNewPlayerEvent(ClientApi.Event.Types.Status status, Hash identity, uint entityId, StdbVector3 startPos, StdbQuaternion startRot, string username)
    {
        if (identity == NetworkPlayer.identity)
        {
            if (status == ClientApi.Event.Types.Status.Committed)
            {
                Debug.Log($"Create player success: {username}");
            }
            else
            {
                UIUsernameChooser.instance.ShowError($"Failed to create user with username: {username}");
            }
        }
    }

    void OnNpcComponentInsert(NpcComponent newValue)
    {
        OnNpcComponentUpdate(null, newValue);
    }

    void OnNpcComponentUpdate(NpcComponent oldValue, NpcComponent newValue)
    {
        // check to see if this player already exists
        if (!npcs.TryGetValue(newValue.entityId, out _))
        {
            // Create a new npc
            var prefabData = npcPrefabs.FirstOrDefault(npcData => npcData.npcType == newValue.model);
            if (prefabData != null)
            {
                var newNpc = Instantiate(prefabData.prefab);
                newNpc.Spawn(newValue.entityId);
                npcs[newValue.entityId] = newNpc;
            }
            else
            {
                Debug.LogError($"Did not find npc prefab for {newValue.model}");
            }
        }
    }

    void OnNpcComponentDelete(NpcComponent oldValue)
    {
        // check to see if this player already exists
        if (npcs.TryGetValue(oldValue.entityId, out var npcModel))
        {
            Destroy(npcModel.gameObject);
            npcs.Remove(oldValue.entityId);
        }
    }

    void OnPlayerComponentInsert(PlayerComponent newValue)
    {
        OnPlayerComponentUpdate(null, newValue);
    }

    void OnPlayerComponentUpdate(PlayerComponent oldValue, PlayerComponent newValue)
    {
        // check to see if this player already exists
        if (!players.TryGetValue(newValue.entityId, out _))
        {
            // Create a new player
            var newNetworkPlayer = Instantiate(playerPrefab);

            // Do we own this player?
            if (NetworkPlayer.identity.HasValue &&
                newValue.ownerId.Equals(NetworkPlayer.identity.Value))
            {
                if (NetworkPlayer.localPlayerId.HasValue)
                {
                    Debug.LogWarning("This identity has more than one player!");
                    return;
                }

                Debug.Log($"Attaching to player with id: {newValue.entityId}");
                NetworkPlayer.localPlayerId = newValue.entityId;
            }

            newNetworkPlayer.Spawn(newValue.entityId);
            players[newValue.entityId] = newNetworkPlayer;
        }
    }

    void OnTransformComponentInsert(TransformComponent newValue)
    {
        OnTransformComponentUpdate(null, newValue);
    }

    void OnTransformComponentUpdate(TransformComponent oldValue, TransformComponent newValue)
    {
        // check to see if this player already exists
        if (players.TryGetValue(newValue.entityId, out var networkPlayer))
        {
            // Is this our player?
            if (networkPlayer.IsLocal())
            {
                // Ignore local updates
            }
            else
            {
                networkPlayer.SetTargetTransform(
                    newValue.pos.ToVector3(),
                    newValue.rot.ToQuaternion()
                );
            }
        }
    }

    void OnAnimationComponentInsert(AnimationComponent newValue)
    {
        OnAnimationComponentUpdate(null, newValue);
    }

    void OnAnimationComponentUpdate(AnimationComponent oldValue, AnimationComponent newValue)
    {
        // check to see if this player already exists
        if (players.TryGetValue(newValue.entityId, out var networkPlayer))
        {
            // Is this our player?
            if (networkPlayer.IsLocal())
            {
                // Ignore local updates
            }
            else
            {
                networkPlayer.GetComponent<PlayerMovementController>().SetMoving(newValue.moving);
                networkPlayer.GetComponentInChildren<PlayerAnimator>(true).SetRemoteAction(newValue.actionTargetEntityId);
            }
        }
    }

    void OnInventoryComponentInsert(InventoryComponent newValue)
    {
        OnInventoryComponentUpdate(null, newValue);
    }

    void OnInventoryComponentUpdate(InventoryComponent oldValue, InventoryComponent newValue)
    {
        // check to see if this player already exists
        if (players.TryGetValue(newValue.entityId, out var networkPlayer))
        {
            // Is this our player?
            if (networkPlayer.IsLocal())
            {
                PlayerInventoryController.Local.InventoryUpdate(newValue);
            }
        }
        else // attempt to update the trade session inventories
        {
            TradeSessionController.Local.InventoryUpdate(newValue);
        }
    }

    void OnPlayerLoginComponentInsert(PlayerLoginComponent newValue)
    {
        OnPlayerLoginComponentUpdate(null, newValue);
    }

    void OnPlayerLoginComponentUpdate(PlayerLoginComponent oldValue, PlayerLoginComponent newValue)
    {
        // check to see if this player already exists
        if (players.TryGetValue(newValue.entityId, out var networkPlayer))
        {
            networkPlayer.LoginStateChanged();
        }
    }

    void OnPlayerChatMessageInsert(PlayerChatMessage newValue)
    {
        OnPlayerChatMessageUpdate(null, newValue);
    }

    void OnPlayerChatMessageUpdate(PlayerChatMessage oldValue, PlayerChatMessage newValue)
    {
        UIChatController.instance.OnChatMessageReceived(newValue.playerId, newValue.message);
    }

    void OnChunkInsert(Chunk newValue)
    {
        OnChunkUpdate(null, newValue);
    }

    void OnChunkUpdate(Chunk oldValue, Chunk newValue)
    {
        TerrainController.instance.AddChunk(newValue);
    }

    void OnResourceComponentInsert(ResourceComponent newValue)
    {
        OnResourceComponentUpdate(null, newValue);
    }

    void OnResourceComponentUpdate(ResourceComponent oldValue, ResourceComponent newValue)
    {
        resources[newValue.entityId] = newValue;
        OnResourceUpdated?.Invoke(newValue.entityId);
    }

    void OnResourceComponentDelete(ResourceComponent oldValue)
    {
        resources.Remove(oldValue.entityId);
        OnResourceUpdated?.Invoke(oldValue.entityId);
    }

    void OnTradeSessionComponentInsert(TradeSessionComponent newValue)
    {
        if (NetworkPlayer.localPlayerId.HasValue)
        {
            var localId = NetworkPlayer.localPlayerId.Value;
            if (newValue.acceptorEntityId == localId || newValue.initiatorEntityId == localId)
            {
                var local = newValue.acceptorEntityId == localId ? newValue.acceptorOfferInventoryEntityId : newValue.initiatorOfferInventoryEntityId;
                var remote = newValue.acceptorEntityId == localId ? newValue.initiatorOfferInventoryEntityId : newValue.acceptorOfferInventoryEntityId;

                TradeSessionController.Local.Initiate(newValue.entityId, local, remote);
            }
        }
    }

    void OnTradeSessionComponentUpdate(TradeSessionComponent oldValue, TradeSessionComponent newValue)
    {
        if (NetworkPlayer.localPlayerId.HasValue)
        {
            var localId = NetworkPlayer.localPlayerId.Value;
            if (newValue.acceptorEntityId == localId || newValue.initiatorEntityId == localId)
            {
                var local = newValue.acceptorEntityId == localId ? newValue.acceptorOfferInventoryEntityId : newValue.initiatorOfferInventoryEntityId;
                var remote = newValue.acceptorEntityId == localId ? newValue.initiatorOfferInventoryEntityId : newValue.acceptorOfferInventoryEntityId;

                TradeSessionController.Local.UpdateSession(newValue.entityId);
            }
        }
    }

    void OnTradeSessionComponentDelete(TradeSessionComponent oldValue)
    {
        var localId = NetworkPlayer.localPlayerId.Value;
        if (oldValue.acceptorEntityId == localId || oldValue.initiatorEntityId == localId)
        {
            TradeSessionController.Local.Terminate(oldValue.approvedByInitiator && oldValue.approvedByAcceptor);
        }
    }
}
