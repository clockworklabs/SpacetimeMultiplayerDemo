using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using Websocket;
using Random = UnityEngine.Random;

public class BitCraftMiniGameManager : Singleton<BitCraftMiniGameManager>
{
    [SerializeField] private NetworkPlayer playerPrefab;
    [SerializeField] private StdbNetworkManager networkManager;
    [SerializeField] private GameObject spawnPosition;
    [SerializeField] private GameObject preSpawnCamera;

    readonly Dictionary<uint, NetworkPlayer> players = new Dictionary<uint, NetworkPlayer>();

    protected void Start()
    {
        networkManager.onConnect += () =>
        {
            try
            {
                Debug.Log("Sending request for new player.");
                var ourId = (uint)(Random.value * uint.MaxValue);
                NetworkPlayer._localPlayerId = ourId;
                Reducer.CreateNewPlayer(spawnPosition.transform.position.ToStdb(), Quaternion.identity.ToStdb());
            }
            catch (Exception e)
            {
                Debug.LogError($"Exception: {e}");
            }
        };
        
        networkManager.onDisconnect += () =>
        {

        };
        
        networkManager.onRowUpdate += (tableId, row) =>
        {
            switch (row.Op)
            {
                case TableRowOperation.Types.OperationType.Insert:
                    if (tableId == 1)
                    {
                        // Player deserialized from row update
                        var rowBytes = row.Row.ToByteArray();
                        var (decoded, _) = TypeValue.Decode(Player.GetTypeDef(), rowBytes, 0, rowBytes.Length);
                        var playerTuple = decoded.Value.GetValue(TypeDef.Def.Tuple) as TypeValue[];
                        var playerId = (uint)playerTuple![0].GetValue(TypeDef.Def.U32);
                        var positionTuple = playerTuple[1].GetValue(TypeDef.Def.Tuple);
                        var rotationTuple = playerTuple[2].GetValue(TypeDef.Def.Tuple);
                        var moving = (bool)playerTuple[3].GetValue(TypeDef.Def.Bool);
                        var positionTupleElements = positionTuple as TypeValue[];
                        Position position = new Position
                        {
                            posX = (float)positionTupleElements![0].GetValue(TypeDef.Def.F32),
                            posY = (float)positionTupleElements![1].GetValue(TypeDef.Def.F32),
                            posZ = (float)positionTupleElements![2].GetValue(TypeDef.Def.F32),
                        };
                        
                        var rotationTupleElements = rotationTuple as TypeValue[];
                        Rotation rotation = new Rotation
                        {
                            rotX = (float)rotationTupleElements![0].GetValue(TypeDef.Def.F32),
                            rotY = (float)rotationTupleElements![1].GetValue(TypeDef.Def.F32),
                            rotZ = (float)rotationTupleElements![2].GetValue(TypeDef.Def.F32),
                            rotW = (float)rotationTupleElements![3].GetValue(TypeDef.Def.F32),
                        };

                        // check to see if this player already exists
                        if (players.TryGetValue(playerId, out var networkPlayer))
                        {
                            // Is this our player?
                            if (networkPlayer.IsLocal())
                            {
                                // Ignore local updates
                            } else
                            {
                                networkPlayer.transform.position = position.ToVector3();
                                networkPlayer.transform.rotation = rotation.ToQuaternion();
                                networkPlayer.GetComponent<PlayerMovementController>().SetMoving(moving);
                            }

                        }
                        else
                        {
                            // Create a new player
                            var newNetworkPlayer = Instantiate(playerPrefab);
                            newNetworkPlayer.transform.position = position.ToVector3();
                            newNetworkPlayer.transform.rotation = rotation.ToQuaternion();
                            newNetworkPlayer.Spawn(playerId);
                            players[playerId] = newNetworkPlayer;
                        }
                    }
                    
                    break;
            }
        };
        
        networkManager.Connect();
    }

    public void LocalPlayerCreated()
    {
        preSpawnCamera.SetActive(false);
    }
}
