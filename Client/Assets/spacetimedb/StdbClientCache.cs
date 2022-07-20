using System;
using System.Collections.Generic;
using Google.Protobuf;
using UnityEngine;
using Websocket;

namespace SpacetimeDB
{
    public class StdbClientCache
    {
        private interface IStdbClientCache
        {
            public bool Insert(ByteString pk, ByteString value);
            
            /// <summary>
            /// Updates an entry. Returns whether or not the update was successful. Updates only succeed if
            /// a previous value was overwritten.
            /// </summary>
            /// <param name="value">The new value</param>
            /// <returns>True when the old value was removed and the new value was inserted.</returns>
            public bool Update(ByteString pk, ByteString value);
            
            /// <summary>
            /// Deletes a value from the table.
            /// </summary>
            /// <param name="value">The value to delete from the table.</param>
            /// <returns></returns>
            public bool Delete(ByteString row_pk, ByteString value);
        }

        private class TableCache
        {
            public readonly string name;
            public readonly uint tableIndex;
            public readonly TypeDef tableRowDef;

            public TableCache(string name, uint tableIndex, TypeDef tableRowDef)
            {
                this.name = name;
                this.tableIndex = tableIndex;
                this.tableRowDef = tableRowDef;
            }

            /// <summary>
            /// Inserts the value into the table. There can be no existing value with the provided pk.
            /// </summary>
            /// <param name="pk">The primary key that uniquely identifies this row</param>
            /// <param name="value">The new value to insert</param>
            /// <returns></returns>
            public TypeValue? Insert(ByteString pk, ByteString value)
            {
                var pk_bytes = pk.ToByteArray();
                if (entries.TryGetValue(pk_bytes, out _))
                {
                    return null;
                }

                var (newValue, _) = TypeValue.Decode(tableRowDef, value);
                if (newValue.HasValue)
                {
                    entries[pk_bytes] = newValue.Value;
                    return newValue.Value;
                }

                // Read failure
                Debug.LogError($"Read error when converting row value for table: {name}:{tableIndex} (version issue?)");
                return null;
            }

            /// <summary>
            /// Updates an entry. Returns whether or not the update was successful. Updates only succeed if
            /// a previous value was overwritten.
            /// </summary>
            /// <param name="pk">The primary key that uniquely identifies this row</param>
            /// <param name="newValueByteString">The new for the table entry</param>
            /// <returns>True when the old value was removed and the new value was inserted.</returns>
            public bool Update(ByteString pk, ByteString newValueByteString)
            {
                // We have to figure out if pk is going to change or not
                throw new InvalidOperationException();
            }

            /// <summary>
            /// Deletes a value from the table.
            /// </summary>
            /// <param name="pk">The primary key that uniquely identifies this row</param>
            /// <returns></returns>
            public TypeValue? Delete(ByteString pk)
            {
                var pk_bytes = pk.ToByteArray();
                if (entries.TryGetValue(pk_bytes, out var value))
                {
                    entries.Remove(pk_bytes);
                    return value;
                }

                return null;
            }

            // Maps from primary key to value
            public readonly Dictionary<byte[], TypeValue> entries = new Dictionary<byte[], TypeValue>();
        }
        
        public enum TableOp
        {
            Insert,
            Delete,
            Update
        }
        
        public delegate void TableUpdated(uint tableIndex, TableOp op, TypeValue? oldValue, TypeValue? newValue);

        public event TableUpdated tableUpdated;

        private readonly Dictionary<uint, TableCache> tables = new Dictionary<uint, TableCache>();

        public void AddTable(string name, uint tableIndex, TypeDef tableRowDef)
        {
            if (tables.TryGetValue(tableIndex, out _))
            {
                Debug.LogError($"Table with index already exists: {tableIndex}");
                return;
            }

            // Initialize this table
            tables[tableIndex] = new TableCache(name, tableIndex, tableRowDef);
        }

        public void ReceiveUpdate(uint tableIndex, TableRowOperation op)
        {
            if (!tables.TryGetValue(tableIndex, out var table))
            {
                Debug.LogError($"We don't know that this table is: {tableIndex}");
                return;
            }

            switch (op.Op)
            {
                case TableRowOperation.Types.OperationType.Delete:
                    var deletedValue = table.Delete(op.RowPk);
                    if (deletedValue.HasValue)
                    {
                        tableUpdated?.Invoke(tableIndex, TableOp.Delete, deletedValue.Value, null);
                    }
                    break;
                case TableRowOperation.Types.OperationType.Insert:
                    var insertedValue = table.Insert(op.RowPk, op.Row); 
                    if (insertedValue.HasValue)
                    {
                        tableUpdated?.Invoke(tableIndex, TableOp.Insert, null, insertedValue.Value);
                    }
                    break;
                case TableRowOperation.Types.OperationType.Update:
                    throw new NotImplementedException();
            }
        }
    }
}