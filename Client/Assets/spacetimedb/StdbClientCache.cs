using System;
using System.Collections.Generic;
using Google.Protobuf;
using UnityEngine;
using Websocket;

namespace SpacetimeDB
{
    public class StdbClientCache
    {
        public StdbClientCache(StdbNetworkManager networkManager)
        {
            networkManager.onRowUpdate += ReceiveUpdate;
        }

        private class TableCache
        {
            public readonly uint tableIndex;
            public readonly TypeDef primaryKeyDef;
            public readonly TypeDef tableRowDef;

            public TableCache(uint tableIndex, TypeDef primaryKeyDef, TypeDef tableRowDef)
            {
                this.tableIndex = tableIndex;
                this.primaryKeyDef = primaryKeyDef;
                this.tableRowDef = tableRowDef;
            }

            // Maps from primary key to value
            public readonly Dictionary<TypeValue, ByteString> entries = new Dictionary<TypeValue, ByteString>();
        }

        public delegate void RowInserted(uint tableIndex, TypeValue newValue);
        public delegate void RowUpdated(uint tableIndex, TypeValue oldValue, TypeValue oldValue, TypeValue newValue);
        public delegate void RowDeleted(uint tableIndex, TypeValue primaryKeyValue, TypeValue rowValue);

        public event RowUpdated rowInserted;
        public event RowUpdated rowDeleted;

        private readonly Dictionary<uint, TableCache> tables = new Dictionary<uint, TableCache>();

        public void AddTable(uint tableIndex, TypeDef primaryKeyDef, TypeDef tableRowDef)
        {
            if (tables.TryGetValue(tableIndex, out var _))
            {
                Debug.LogError($"Table with index already exists: {tableIndex}");
                return;
            }

            // Initialize this table
            tables[tableIndex] = new TableCache(tableIndex, primaryKeyDef, tableRowDef);
        }

        public void ReceiveUpdate(uint tableIndex, TableRowOperation op)
        {
            if (!tables.TryGetValue(tableIndex, out var table))
            {
                Debug.LogError($"We don't know that this table is: {tableIndex}");
                return;
            }

            var primaryKeyBuffer = op.RowPk.ToByteArray();
            var (primaryKey, _) = TypeValue.Decode(Player.GetTypeDef(), primaryKeyBuffer, 0, primaryKeyBuffer.Length);

            if (!primaryKey.HasValue)
            {
                return;
            }

            switch (op.Op)
            {
                case TableRowOperation.Types.OperationType.Delete:
                    table.entries.Remove(primaryKey.Value);
                    break;
                case TableRowOperation.Types.OperationType.Insert:
                    table.entries[primaryKey.Value] = op.RowPk;
                    rowInserted?.Invoke(tableIndex, value);
                    break;
            }
            
            
        }
    }
}