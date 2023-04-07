// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;

namespace SpacetimeDB
{
	public partial class NpcComponent : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("entity_id")]
		public ulong EntityId;
		[Newtonsoft.Json.JsonProperty("model")]
		public string Model;
		[Newtonsoft.Json.JsonProperty("next_action")]
		public ulong NextAction;

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("entity_id", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64)),
				new SpacetimeDB.SATS.ProductTypeElement("model", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.String)),
				new SpacetimeDB.SATS.ProductTypeElement("next_action", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64)),
			});
		}

		public static explicit operator NpcComponent(SpacetimeDB.SATS.AlgebraicValue value)
		{
			if (value == null) {
				return null;
			}
			var productValue = value.AsProductValue();
			return new NpcComponent
			{
				EntityId = productValue.elements[0].AsU64(),
				Model = productValue.elements[1].AsString(),
				NextAction = productValue.elements[2].AsU64(),
			};
		}

		public static System.Collections.Generic.IEnumerable<NpcComponent> Iter()
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("NpcComponent"))
			{
				yield return (NpcComponent)entry.Item2;
			}
		}
		public static int Count()
		{
			return NetworkManager.clientDB.Count("NpcComponent");
		}
		public static NpcComponent FilterByEntityId(ulong value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("NpcComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (ulong)productValue.elements[0].AsU64();
				if (compareValue == value) {
					return (NpcComponent)entry.Item2;
				}
			}
			return null;
		}

		public static System.Collections.Generic.IEnumerable<NpcComponent> FilterByModel(string value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("NpcComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (string)productValue.elements[1].AsString();
				if (compareValue == value) {
					yield return (NpcComponent)entry.Item2;
				}
			}
		}

		public static System.Collections.Generic.IEnumerable<NpcComponent> FilterByNextAction(ulong value)
		{
			foreach(var entry in NetworkManager.clientDB.GetEntries("NpcComponent"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (ulong)productValue.elements[2].AsU64();
				if (compareValue == value) {
					yield return (NpcComponent)entry.Item2;
				}
			}
		}

		public static event Action<NpcComponent> OnInsert;
		public static event Action<NpcComponent, NpcComponent> OnUpdate;
		public static event Action<NpcComponent> OnDelete;
		public static event Action<NetworkManager.TableOp, NpcComponent, NpcComponent> OnRowUpdate;

		public static void OnInsertEvent(object newValue)
		{
			OnInsert?.Invoke((NpcComponent)newValue);
		}

		public static void OnUpdateEvent(object oldValue, object newValue)
		{
			OnUpdate?.Invoke((NpcComponent)oldValue,(NpcComponent)newValue);
		}

		public static void OnDeleteEvent(object oldValue)
		{
			OnDelete?.Invoke((NpcComponent)oldValue);
		}

		public static void OnRowUpdateEvent(NetworkManager.TableOp op, object oldValue, object newValue)
		{
			OnRowUpdate?.Invoke(op, (NpcComponent)oldValue,(NpcComponent)newValue);
		}
	}
}
