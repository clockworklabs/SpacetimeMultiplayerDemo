// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;

namespace SpacetimeDB
{
	public partial class MobileEntityComponent : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("entity_id")]
		public ulong EntityId;
		[Newtonsoft.Json.JsonProperty("location")]
		public SpacetimeDB.StdbVector2 Location;
		[Newtonsoft.Json.JsonProperty("direction")]
		public SpacetimeDB.StdbVector2 Direction;
		[Newtonsoft.Json.JsonProperty("move_start_timestamp")]
		public ulong MoveStartTimestamp;

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("entity_id", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64)),
				new SpacetimeDB.SATS.ProductTypeElement("location", SpacetimeDB.StdbVector2.GetAlgebraicType()),
				new SpacetimeDB.SATS.ProductTypeElement("direction", SpacetimeDB.StdbVector2.GetAlgebraicType()),
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
				Location = (SpacetimeDB.StdbVector2)(productValue.elements[1]),
				Direction = (SpacetimeDB.StdbVector2)(productValue.elements[2]),
				MoveStartTimestamp = productValue.elements[3].AsU64(),
			};
		}

		public static System.Collections.Generic.IEnumerable<MobileEntityComponent> Iter()
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("MobileEntityComponent"))
			{
				yield return (MobileEntityComponent)entry.Item2;
			}
		}
		public static int Count()
		{
			return NetworkManager.clientDB.Count("MobileEntityComponent");
		}
		public static MobileEntityComponent FilterByEntityId(ulong value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("MobileEntityComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (ulong)productValue.elements[0].AsU64();
				if (compareValue == value) {
					return (MobileEntityComponent)entry.Item2;
				}
			}
			return null;
		}

		public static System.Collections.Generic.IEnumerable<MobileEntityComponent> FilterByMoveStartTimestamp(ulong value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("MobileEntityComponent"))
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

		public delegate void InsertEventHandler(MobileEntityComponent insertedValue, ClientApi.Event dbEvent);
		public delegate void UpdateEventHandler(MobileEntityComponent oldValue, MobileEntityComponent newValue, ClientApi.Event dbEvent);
		public delegate void DeleteEventHandler(MobileEntityComponent deletedValue, ClientApi.Event dbEvent);
		public delegate void RowUpdateEventHandler(NetworkManager.TableOp op, MobileEntityComponent oldValue, MobileEntityComponent newValue, ClientApi.Event dbEvent);
		public static event InsertEventHandler OnInsert;
		public static event UpdateEventHandler OnUpdate;
		public static event DeleteEventHandler OnDelete;
		public static event RowUpdateEventHandler OnRowUpdate;

		public static void OnInsertEvent(object newValue, ClientApi.Event dbEvent)
		{
			OnInsert?.Invoke((MobileEntityComponent)newValue,dbEvent);
		}

		public static void OnUpdateEvent(object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnUpdate?.Invoke((MobileEntityComponent)oldValue,(MobileEntityComponent)newValue,dbEvent);
		}

		public static void OnDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnDelete?.Invoke((MobileEntityComponent)oldValue,dbEvent);
		}

		public static void OnRowUpdateEvent(NetworkManager.TableOp op, object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnRowUpdate?.Invoke(op, (MobileEntityComponent)oldValue,(MobileEntityComponent)newValue,dbEvent);
		}
	}
}
