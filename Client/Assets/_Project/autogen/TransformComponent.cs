// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;

namespace SpacetimeDB
{
	public partial class TransformComponent : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("entity_id")]
		public ulong EntityId;
		[Newtonsoft.Json.JsonProperty("pos")]
		public SpacetimeDB.StdbVector3 Pos;
		[Newtonsoft.Json.JsonProperty("rot")]
		public SpacetimeDB.StdbQuaternion Rot;

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("entity_id", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64)),
				new SpacetimeDB.SATS.ProductTypeElement("pos", SpacetimeDB.StdbVector3.GetAlgebraicType()),
				new SpacetimeDB.SATS.ProductTypeElement("rot", SpacetimeDB.StdbQuaternion.GetAlgebraicType()),
			});
		}

		public static explicit operator TransformComponent(SpacetimeDB.SATS.AlgebraicValue value)
		{
			if (value == null) {
				return null;
			}
			var productValue = value.AsProductValue();
			return new TransformComponent
			{
				EntityId = productValue.elements[0].AsU64(),
				Pos = (SpacetimeDB.StdbVector3)(productValue.elements[1]),
				Rot = (SpacetimeDB.StdbQuaternion)(productValue.elements[2]),
			};
		}

		public static System.Collections.Generic.IEnumerable<TransformComponent> Iter()
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("TransformComponent"))
			{
				yield return (TransformComponent)entry.Item2;
			}
		}
		public static int Count()
		{
			return NetworkManager.clientDB.Count("TransformComponent");
		}
		public static TransformComponent FilterByEntityId(ulong value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("TransformComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (ulong)productValue.elements[0].AsU64();
				if (compareValue == value) {
					return (TransformComponent)entry.Item2;
				}
			}
			return null;
		}

		public static event Action<TransformComponent> OnInsert;
		public static event Action<TransformComponent, TransformComponent> OnUpdate;
		public static event Action<TransformComponent> OnDelete;
		public static event Action<NetworkManager.TableOp, TransformComponent, TransformComponent> OnRowUpdate;

		public static void OnInsertEvent(object newValue)
		{
			OnInsert?.Invoke((TransformComponent)newValue);
		}

		public static void OnUpdateEvent(object oldValue, object newValue)
		{
			OnUpdate?.Invoke((TransformComponent)oldValue,(TransformComponent)newValue);
		}

		public static void OnDeleteEvent(object oldValue)
		{
			OnDelete?.Invoke((TransformComponent)oldValue);
		}

		public static void OnRowUpdateEvent(NetworkManager.TableOp op, object oldValue, object newValue)
		{
			OnRowUpdate?.Invoke(op, (TransformComponent)oldValue,(TransformComponent)newValue);
		}
	}
}