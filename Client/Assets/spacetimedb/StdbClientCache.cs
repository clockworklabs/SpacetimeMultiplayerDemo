using System;
using System.Collections.Generic;
using System.Linq;
using Google.Protobuf;
using UnityEngine;
using ClientApi;

namespace SpacetimeDB
{
    public class StdbClientCache
    {
        public class TableCache
        {
            private class ByteArrayComparer : IEqualityComparer<byte[]> {
                public bool Equals(byte[] left, byte[] right) {
                    if ( left == null || right == null ) {
                        return left == right;
                    }
                    return left.SequenceEqual(right);
                }
                public int GetHashCode(byte[] key) {
                    if (key == null)
                        throw new ArgumentNullException(nameof(key));
                    return key.Sum(b => b);
                }
            }

            private readonly string name;
            private readonly TypeDef tableRowDef;

            public TableCache(string name, TypeDef tableRowDef)
            {
                this.name = name;
                this.tableRowDef = tableRowDef;
                entries = new Dictionary<byte[], TypeValue>(new ByteArrayComparer());
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
                Debug.LogError($"Read error when converting row value for table: {name} (version issue?)");
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
            public readonly Dictionary<byte[], TypeValue> entries;
        }

        private readonly Dictionary<string, TableCache> tables = new Dictionary<string, TableCache>();

        public void AddTable(string name, TypeDef tableRowDef)
        {
            if (tables.TryGetValue(name, out _))
            {
                Debug.LogError($"Table with name already exists: {name}");
                return;
            }

            // Initialize this table
            tables[name] = new TableCache(name, tableRowDef);
        }

        public IEnumerable<TypeValue> GetEntries(string name)
        {
            if (!tables.TryGetValue(name, out var table))
            {
                yield break;
            }

            foreach (var entry in table.entries)
            {
                yield return entry.Value;
            }
        }

        /// <summary>
        /// Updates the given table with the given operation. If an entry is deleted due to this operation, it is returned.
        /// </summary>
        /// <param name="name">The name of the table the update is for.</param>
        /// <param name="op">The operation on the table row</param>
        /// <returns>The deleted value, if there is one.</returns>
        public TypeValue? ReceiveUpdate(string name, TableRowOperation op)
        {
            if (!tables.TryGetValue(name, out var table))
            {
                Debug.LogError($"We don't know that this table is: {name}");
                return null;
            }

            switch (op.Op)
            {
                case TableRowOperation.Types.OperationType.Delete:
                    return table.Delete(op.RowPk);
                case TableRowOperation.Types.OperationType.Insert:
                    table.Insert(op.RowPk, op.Row);
                    break;
            }
            
            return null;
        }

        public TableCache GetTable(string name)
        {
            if (tables.TryGetValue(name, out var table))
            {
                return table;
            }
            
            Debug.LogError($"We don't know that this table is: {name}");
            return null;
        }
    }
}