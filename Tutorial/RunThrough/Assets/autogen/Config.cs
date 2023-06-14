// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;

namespace SpacetimeDB
{
	public partial class Config : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("version")]
		public uint Version;
		[Newtonsoft.Json.JsonProperty("message_of_the_day")]
		public string MessageOfTheDay;
		[Newtonsoft.Json.JsonProperty("map_extents")]
		public uint MapExtents;
		[Newtonsoft.Json.JsonProperty("num_resource_nodes")]
		public uint NumResourceNodes;

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("version", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U32)),
				new SpacetimeDB.SATS.ProductTypeElement("message_of_the_day", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.String)),
				new SpacetimeDB.SATS.ProductTypeElement("map_extents", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U32)),
				new SpacetimeDB.SATS.ProductTypeElement("num_resource_nodes", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U32)),
			});
		}

		public static explicit operator Config(SpacetimeDB.SATS.AlgebraicValue value)
		{
			if (value == null) {
				return null;
			}
			var productValue = value.AsProductValue();
			return new Config
			{
				Version = productValue.elements[0].AsU32(),
				MessageOfTheDay = productValue.elements[1].AsString(),
				MapExtents = productValue.elements[2].AsU32(),
				NumResourceNodes = productValue.elements[3].AsU32(),
			};
		}

		public static System.Collections.Generic.IEnumerable<Config> Iter()
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("Config"))
			{
				yield return (Config)entry.Item2;
			}
		}
		public static int Count()
		{
			return NetworkManager.clientDB.Count("Config");
		}
		public static Config FilterByVersion(uint value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("Config"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (uint)productValue.elements[0].AsU32();
				if (compareValue == value) {
					return (Config)entry.Item2;
				}
			}
			return null;
		}

		public static System.Collections.Generic.IEnumerable<Config> FilterByMessageOfTheDay(string value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("Config"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (string)productValue.elements[1].AsString();
				if (compareValue == value) {
					yield return (Config)entry.Item2;
				}
			}
		}

		public static System.Collections.Generic.IEnumerable<Config> FilterByMapExtents(uint value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("Config"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (uint)productValue.elements[2].AsU32();
				if (compareValue == value) {
					yield return (Config)entry.Item2;
				}
			}
		}

		public static System.Collections.Generic.IEnumerable<Config> FilterByNumResourceNodes(uint value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("Config"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (uint)productValue.elements[3].AsU32();
				if (compareValue == value) {
					yield return (Config)entry.Item2;
				}
			}
		}

		public static bool ComparePrimaryKey(SpacetimeDB.SATS.AlgebraicType t, SpacetimeDB.SATS.AlgebraicValue v1, SpacetimeDB.SATS.AlgebraicValue v2)
		{
			var primaryColumnValue1 = v1.AsProductValue().elements[0];
			var primaryColumnValue2 = v2.AsProductValue().elements[0];
			return SpacetimeDB.SATS.AlgebraicValue.Compare(t.product.elements[0].algebraicType, primaryColumnValue1, primaryColumnValue2);
		}

		public delegate void InsertEventHandler(Config insertedValue, ClientApi.Event dbEvent);
		public delegate void UpdateEventHandler(Config oldValue, Config newValue, ClientApi.Event dbEvent);
		public delegate void DeleteEventHandler(Config deletedValue, ClientApi.Event dbEvent);
		public delegate void RowUpdateEventHandler(NetworkManager.TableOp op, Config oldValue, Config newValue, ClientApi.Event dbEvent);
		public static event InsertEventHandler OnInsert;
		public static event UpdateEventHandler OnUpdate;
		public static event DeleteEventHandler OnDelete;
		public static event RowUpdateEventHandler OnRowUpdate;

		public static void OnInsertEvent(object newValue, ClientApi.Event dbEvent)
		{
			OnInsert?.Invoke((Config)newValue,dbEvent);
		}

		public static void OnUpdateEvent(object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnUpdate?.Invoke((Config)oldValue,(Config)newValue,dbEvent);
		}

		public static void OnDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnDelete?.Invoke((Config)oldValue,dbEvent);
		}

		public static void OnRowUpdateEvent(NetworkManager.TableOp op, object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnRowUpdate?.Invoke(op, (Config)oldValue,(Config)newValue,dbEvent);
		}
	}
}