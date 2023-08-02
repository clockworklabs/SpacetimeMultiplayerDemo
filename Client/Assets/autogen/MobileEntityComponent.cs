// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;
using System.Collections.Generic;
using SpacetimeDB;

namespace SpacetimeDB.Types
{
	public partial class MobileEntityComponent : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("entity_id")]
		public ulong EntityId;
		[Newtonsoft.Json.JsonProperty("location")]
		public SpacetimeDB.Types.StdbVector2 Location;
		[Newtonsoft.Json.JsonProperty("direction")]
		public SpacetimeDB.Types.StdbVector2 Direction;
		[Newtonsoft.Json.JsonProperty("move_start_timestamp")]
		public ulong MoveStartTimestamp;

		private static Dictionary<ulong, MobileEntityComponent> EntityId_Index = new Dictionary<ulong, MobileEntityComponent>(16);

		private static void InternalOnValueInserted(object insertedValue)
		{
			var val = (MobileEntityComponent)insertedValue;
			EntityId_Index[val.EntityId] = val;
		}

		private static void InternalOnValueDeleted(object deletedValue)
		{
			var val = (MobileEntityComponent)deletedValue;
			EntityId_Index.Remove(val.EntityId);
		}

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("entity_id", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64)),
				new SpacetimeDB.SATS.ProductTypeElement("location", SpacetimeDB.Types.StdbVector2.GetAlgebraicType()),
				new SpacetimeDB.SATS.ProductTypeElement("direction", SpacetimeDB.Types.StdbVector2.GetAlgebraicType()),
				new SpacetimeDB.SATS.ProductTypeElement("move_start_timestamp", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64)),
			});
		}

		public static explicit operator MobileEntityComponent(SpacetimeDB.SATS.AlgebraicValue value)
		{
			if (value == null) {
				return null;
			}
			var productValue = value.AsProductValue();
			return new MobileEntityComponent
			{
				EntityId = productValue.elements[0].AsU64(),
				Location = (SpacetimeDB.Types.StdbVector2)(productValue.elements[1]),
				Direction = (SpacetimeDB.Types.StdbVector2)(productValue.elements[2]),
				MoveStartTimestamp = productValue.elements[3].AsU64(),
			};
		}

		public static System.Collections.Generic.IEnumerable<MobileEntityComponent> Iter()
		{
			foreach(var entry in SpacetimeDBClient.clientDB.GetEntries("MobileEntityComponent"))
			{
				yield return (MobileEntityComponent)entry.Item2;
			}
		}
		public static int Count()
		{
			return SpacetimeDBClient.clientDB.Count("MobileEntityComponent");
		}
		public static MobileEntityComponent FilterByEntityId(ulong value)
		{
			EntityId_Index.TryGetValue(value, out var r);
			return r;
		}

		public static System.Collections.Generic.IEnumerable<MobileEntityComponent> FilterByMoveStartTimestamp(ulong value)
		{
			foreach(var entry in SpacetimeDBClient.clientDB.GetEntries("MobileEntityComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (ulong)productValue.elements[3].AsU64();
				if (compareValue == value) {
					yield return (MobileEntityComponent)entry.Item2;
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

		public delegate void InsertEventHandler(MobileEntityComponent insertedValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void UpdateEventHandler(MobileEntityComponent oldValue, MobileEntityComponent newValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void DeleteEventHandler(MobileEntityComponent deletedValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void RowUpdateEventHandler(SpacetimeDBClient.TableOp op, MobileEntityComponent oldValue, MobileEntityComponent newValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public static event InsertEventHandler OnInsert;
		public static event UpdateEventHandler OnUpdate;
		public static event DeleteEventHandler OnBeforeDelete;
		public static event DeleteEventHandler OnDelete;
		public static event RowUpdateEventHandler OnRowUpdate;

		public static void OnInsertEvent(object newValue, ClientApi.Event dbEvent)
		{
			OnInsert?.Invoke((MobileEntityComponent)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnUpdateEvent(object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnUpdate?.Invoke((MobileEntityComponent)oldValue,(MobileEntityComponent)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnBeforeDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnBeforeDelete?.Invoke((MobileEntityComponent)oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnDelete?.Invoke((MobileEntityComponent)oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnRowUpdateEvent(SpacetimeDBClient.TableOp op, object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnRowUpdate?.Invoke(op, (MobileEntityComponent)oldValue,(MobileEntityComponent)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}
	}
}
