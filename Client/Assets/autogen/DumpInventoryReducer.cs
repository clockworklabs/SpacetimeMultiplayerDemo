// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;
using ClientApi;
using Newtonsoft.Json.Linq;
using SpacetimeDB;

namespace SpacetimeDB.Types
{
	public static partial class Reducer
	{
		public delegate void DumpInventoryHandler(ReducerEvent reducerEvent, ulong entityId);
		public static event DumpInventoryHandler OnDumpInventoryEvent;

		public static void DumpInventory(ulong entityId)
		{
			var _argArray = new object[] {entityId};
			var _message = new SpacetimeDBClient.ReducerCallRequest {
				fn = "dump_inventory",
				args = _argArray,
			};
			Newtonsoft.Json.JsonSerializerSettings _settings = new Newtonsoft.Json.JsonSerializerSettings
			{
				Converters = { new SpacetimeDB.SomeWrapperConverter(), new SpacetimeDB.EnumWrapperConverter() },
				ContractResolver = new SpacetimeDB.JsonContractResolver(),
			};
			SpacetimeDBClient.instance.InternalCallReducer(Newtonsoft.Json.JsonConvert.SerializeObject(_message, _settings));
		}

		[ReducerCallback(FunctionName = "dump_inventory")]
		public static bool OnDumpInventory(ClientApi.Event dbEvent)
		{
			if(OnDumpInventoryEvent != null)
			{
				var args = ((ReducerEvent)dbEvent.FunctionCall.CallInfo).DumpInventoryArgs;
				OnDumpInventoryEvent((ReducerEvent)dbEvent.FunctionCall.CallInfo
					,(ulong)args.EntityId
				);
				return true;
			}
			return false;
		}

		[DeserializeEvent(FunctionName = "dump_inventory")]
		public static void DumpInventoryDeserializeEventArgs(ClientApi.Event dbEvent)
		{
			var args = new DumpInventoryArgsStruct();
			var bsatnBytes = dbEvent.FunctionCall.ArgBytes;
			using var ms = new System.IO.MemoryStream();
			ms.SetLength(bsatnBytes.Length);
			bsatnBytes.CopyTo(ms.GetBuffer(), 0);
			ms.Position = 0;
			using var reader = new System.IO.BinaryReader(ms);
			var args_0_value = SpacetimeDB.SATS.AlgebraicValue.Deserialize(SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U64), reader);
			args.EntityId = args_0_value.AsU64();
			dbEvent.FunctionCall.CallInfo = new ReducerEvent(ReducerType.DumpInventory, "dump_inventory", dbEvent.Timestamp, Identity.From(dbEvent.CallerIdentity.ToByteArray()), dbEvent.Message, dbEvent.Status, args);
		}
	}

	public partial class DumpInventoryArgsStruct
	{
		public ulong EntityId;
	}

}
