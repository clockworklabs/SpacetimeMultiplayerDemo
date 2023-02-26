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

    void OnCreateNewPlayerEvent(ClientApi.Event.Types.Status status, Identity identity, StdbVector3 startPos, StdbQuaternion startRot, string username)
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
        if (!npcs.TryGetValue(newValue.EntityId, out _))
        {
            // Create a new npc
            var prefabData = npcPrefabs.FirstOrDefault(npcData => npcData.npcType == newValue.Model);
            if (prefabData != null)
            {
                var newNpc = Instantiate(prefabData.prefab);
                newNpc.Spawn(newValue.EntityId);
                npcs[newValue.EntityId] = newNpc;
            }
            else
            {
                Debug.LogError($"Did not find npc prefab for {newValue.Model}");
            }
        }
    }

    void OnNpcComponentDelete(NpcComponent oldValue)
    {
        // check to see if this player already exists
        if (npcs.TryGetValue(oldValue.EntityId, out var npcModel))
        {
            Destroy(npcModel.gameObject);
            npcs.Remove(oldValue.EntityId);
        }
    }

    void OnPlayerComponentInsert(PlayerComponent newValue)
    {
        OnPlayerComponentUpdate(null, newValue);
    }

    void OnPlayerComponentUpdate(PlayerComponent oldValue, PlayerComponent newValue)
    {
        // check to see if this player already exists
        if (!players.TryGetValue(newValue.EntityId, out _))
        {
            // Create a new player
            var newNetworkPlayer = Instantiate(playerPrefab);

            // Do we own this player?
            if (NetworkPlayer.identity.HasValue &&
                Identity.From(newValue.OwnerId).Equals(NetworkPlayer.identity.Value))
            {
                if (NetworkPlayer.localPlayerId.HasValue)
                {
                    Debug.LogWarning("This identity has more than one player!");
                    return;
                }

                Debug.Log($"Attaching to player with id: {newValue.EntityId}");
                NetworkPlayer.localPlayerId = newValue.EntityId;
            }

            newNetworkPlayer.Spawn(newValue.EntityId);
            players[newValue.EntityId] = newNetworkPlayer;
        }
    }

    void OnTransformComponentInsert(TransformComponent newValue)
    {
        OnTransformComponentUpdate(null, newValue);
    }

    void OnTransformComponentUpdate(TransformComponent oldValue, TransformComponent newValue)
    {
        // check to see if this player already exists
        if (players.TryGetValue(newValue.EntityId, out var networkPlayer))
        {
            // Is this our player?
            if (networkPlayer.IsLocal())
            {
                // Ignore local updates
            }
            else
            {
                networkPlayer.SetTargetTransform(
                    newValue.Pos.ToVector3(),
                    newValue.Rot.ToQuaternion()
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
        if (players.TryGetValue(newValue.EntityId, out var networkPlayer))
        {
            // Is this our player?
            if (networkPlayer.IsLocal())
            {
                // Ignore local updates
            }
            else
            {
                networkPlayer.GetComponent<PlayerMovementController>().SetMoving(newValue.Moving);
                networkPlayer.GetComponentInChildren<PlayerAnimator>(true).SetRemoteAction(newValue.ActionTargetEntityId);
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
        if (players.TryGetValue(newValue.EntityId, out var networkPlayer))
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
        if (players.TryGetValue(newValue.EntityId, out var networkPlayer))
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
        UIChatController.instance.OnChatMessageReceived(newValue.PlayerId, newValue.Message);
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
        resources[newValue.EntityId] = newValue;
        OnResourceUpdated?.Invoke(newValue.EntityId);
    }

    void OnResourceComponentDelete(ResourceComponent oldValue)
    {
        resources.Remove(oldValue.EntityId);
        OnResourceUpdated?.Invoke(oldValue.EntityId);
    }

    void OnTradeSessionComponentInsert(TradeSessionComponent newValue)
    {
        if (NetworkPlayer.localPlayerId.HasValue)
        {
            var localId = NetworkPlayer.localPlayerId.Value;
            if (newValue.AcceptorEntityId == localId || newValue.InitiatorEntityId == localId)
            {
                var local = newValue.AcceptorEntityId == localId ? newValue.AcceptorOfferInventoryEntityId : newValue.InitiatorOfferInventoryEntityId;
                var remote = newValue.AcceptorEntityId == localId ? newValue.InitiatorOfferInventoryEntityId : newValue.AcceptorOfferInventoryEntityId;

                TradeSessionController.Local.Initiate(newValue.EntityId, local, remote);
            }
        }
    }

    void OnTradeSessionComponentUpdate(TradeSessionComponent oldValue, TradeSessionComponent newValue)
    {
        if (NetworkPlayer.localPlayerId.HasValue)
        {
            var localId = NetworkPlayer.localPlayerId.Value;
            if (newValue.AcceptorEntityId == localId || newValue.InitiatorEntityId == localId)
            {
                var local = newValue.AcceptorEntityId == localId ? newValue.AcceptorOfferInventoryEntityId : newValue.InitiatorOfferInventoryEntityId;
                var remote = newValue.AcceptorEntityId == localId ? newValue.InitiatorOfferInventoryEntityId : newValue.AcceptorOfferInventoryEntityId;

                TradeSessionController.Local.UpdateSession(newValue.EntityId);
            }
        }
    }

    void OnTradeSessionComponentDelete(TradeSessionComponent oldValue)
    {
        var localId = NetworkPlayer.localPlayerId.Value;
        if (oldValue.AcceptorEntityId == localId || oldValue.InitiatorEntityId == localId)
        {
            TradeSessionController.Local.Terminate(oldValue.ApprovedByInitiator && oldValue.ApprovedByAcceptor);
        }
    }
}
