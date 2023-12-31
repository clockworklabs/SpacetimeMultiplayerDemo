// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;
using System.Collections.Generic;
using SpacetimeDB;

namespace SpacetimeDB.Types
{
	public partial class PlayerComponent : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("entity_id")]
		public ulong EntityId;
		[Newtonsoft.Json.JsonProperty("owner_id")]
		public SpacetimeDB.Identity OwnerId;
		[Newtonsoft.Json.JsonProperty("username")]
		public string Username;
		[Newtonsoft.Json.JsonProperty("logged_in")]
		public bool LoggedIn;

		private static Dictionary<ulong, PlayerComponent> EntityId_Index = new Dictionary<ulong, PlayerComponent>(16);
		private static Dictionary<SpacetimeDB.Identity, PlayerComponent> OwnerId_Index = new Dictionary<SpacetimeDB.Identity, PlayerComponent>(16);

		private static void InternalOnValueInserted(object insertedValue)
		{
			var val = (PlayerComponent)insertedValue;
			EntityId_Index[val.EntityId] = val;
			OwnerId_Index[val.OwnerId] = val;
		}

		private static void InternalOnValueDeleted(object deletedValue)
		{
			var val = (PlayerComponent)deletedValue;
			EntityId_Index.Remove(val.EntityId);
			OwnerId_Index.Remove(val.OwnerId);
		}

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("entity_id", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64)),
				new SpacetimeDB.SATS.ProductTypeElement("owner_id", SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("__identity_bytes", SpacetimeDB.SATS.AlgebraicType.CreateArrayType(SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U8))),
			})),
				new SpacetimeDB.SATS.ProductTypeElement("username", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.String)),
				new SpacetimeDB.SATS.ProductTypeElement("logged_in", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.Bool)),
			});
		}

		public static explicit operator PlayerComponent(SpacetimeDB.SATS.AlgebraicValue value)
		{
			if (value == null) {
				return null;
			}
			var productValue = value.AsProductValue();
			return new PlayerComponent
			{
				EntityId = productValue.elements[0].AsU64(),
				OwnerId = SpacetimeDB.Identity.From(productValue.elements[1].AsProductValue().elements[0].AsBytes()),
				Username = productValue.elements[2].AsString(),
				LoggedIn = productValue.elements[3].AsBool(),
			};
		}

		public static System.Collections.Generic.IEnumerable<PlayerComponent> Iter()
		{
			foreach(var entry in SpacetimeDBClient.clientDB.GetEntries("PlayerComponent"))
			{
				yield return (PlayerComponent)entry.Item2;
			}
		}
		public static int Count()
		{
			return SpacetimeDBClient.clientDB.Count("PlayerComponent");
		}
		public static PlayerComponent FilterByEntityId(ulong value)
		{
			EntityId_Index.TryGetValue(value, out var r);
			return r;
		}

		public static PlayerComponent FilterByOwnerId(SpacetimeDB.Identity value)
		{
			OwnerId_Index.TryGetValue(value, out var r);
			return r;
		}

		public static System.Collections.Generic.IEnumerable<PlayerComponent> FilterByUsername(string value)
		{
			foreach(var entry in SpacetimeDBClient.clientDB.GetEntries("PlayerComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (string)productValue.elements[2].AsString();
				if (compareValue == value) {
					yield return (PlayerComponent)entry.Item2;
				}
			}
		}

		public static System.Collections.Generic.IEnumerable<PlayerComponent> FilterByLoggedIn(bool value)
		{
			foreach(var entry in SpacetimeDBClient.clientDB.GetEntries("PlayerComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (bool)productValue.elements[3].AsBool();
				if (compareValue == value) {
					yield return (PlayerComponent)entry.Item2;
				}
			}
		}

		public static bool ComparePrimaryKey(SpacetimeDB.SATS.AlgebraicType t, SpacetimeDB.SATS.AlgebraicValue v1, SpacetimeDB.SATS.AlgebraicValue v2)
		{
			var primaryColumnValue1 = v1.AsProductValue().elements[0];
			var primaryColumnValue2 = v2.AsProductValue().elements[0];
			return SpacetimeDB.SATS.AlgebraicValue.Compare(t.product.elements[0].algebraicType, primaryColumnValue1, primaryColumnValue2);
		}

		public static SpacetimeDB.SATS.AlgebraicValue GetPrimaryKeyValue(SpacetimeDB.SATS.AlgebraicValue v)
		{
			return v.AsProductValue().elements[0];
		}

		public static SpacetimeDB.SATS.AlgebraicType GetPrimaryKeyType(SpacetimeDB.SATS.AlgebraicType t)
		{
			return t.product.elements[0].algebraicType;
		}

		public delegate void InsertEventHandler(PlayerComponent insertedValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void UpdateEventHandler(PlayerComponent oldValue, PlayerComponent newValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void DeleteEventHandler(PlayerComponent deletedValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void RowUpdateEventHandler(SpacetimeDBClient.TableOp op, PlayerComponent oldValue, PlayerComponent newValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public static event InsertEventHandler OnInsert;
		public static event UpdateEventHandler OnUpdate;
		public static event DeleteEventHandler OnBeforeDelete;
		public static event DeleteEventHandler OnDelete;
		public static event RowUpdateEventHandler OnRowUpdate;

		public static void OnInsertEvent(object newValue, ClientApi.Event dbEvent)
		{
			OnInsert?.Invoke((PlayerComponent)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnUpdateEvent(object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnUpdate?.Invoke((PlayerComponent)oldValue,(PlayerComponent)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnBeforeDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnBeforeDelete?.Invoke((PlayerComponent)oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnDelete?.Invoke((PlayerComponent)oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnRowUpdateEvent(SpacetimeDBClient.TableOp op, object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnRowUpdate?.Invoke(op, (PlayerComponent)oldValue,(PlayerComponent)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}
	}
}
