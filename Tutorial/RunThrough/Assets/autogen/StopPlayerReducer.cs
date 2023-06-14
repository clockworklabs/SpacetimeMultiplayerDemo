// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;
using ClientApi;
using Newtonsoft.Json.Linq;

namespace SpacetimeDB
{
	public static partial class Reducer
	{
		public static event Action<ClientApi.Event.Types.Status, Identity, SpacetimeDB.StdbVector2> OnStopPlayerEvent;

		public static void StopPlayer(SpacetimeDB.StdbVector2 location)
		{
			var _argArray = new object[] {location};
			var _message = new NetworkManager.ReducerCallRequest {
				fn = "stop_player",
				args = _argArray,
			};
			Newtonsoft.Json.JsonSerializerSettings _settings = new Newtonsoft.Json.JsonSerializerSettings
			{
				Converters = { new SpacetimeDB.SomeWrapperConverter(), new SpacetimeDB.EnumWrapperConverter() },
				ContractResolver = new SpacetimeDB.JsonContractResolver(),
			};
			NetworkManager.instance.InternalCallReducer(Newtonsoft.Json.JsonConvert.SerializeObject(_message, _settings));
		}

		[ReducerEvent(FunctionName = "stop_player")]
		public static void OnStopPlayer(ClientApi.Event dbEvent)
		{
			if(OnStopPlayerEvent != null)
			{
				var bsatnBytes = dbEvent.FunctionCall.ArgBytes;
				using var ms = new System.IO.MemoryStream();
				ms.SetLength(bsatnBytes.Length);
				bsatnBytes.CopyTo(ms.GetBuffer(), 0);
				ms.Position = 0;
				using var reader = new System.IO.BinaryReader(ms);
				var args_0_value = SpacetimeDB.SATS.AlgebraicValue.Deserialize(SpacetimeDB.StdbVector2.GetAlgebraicType(), reader);
				var args_0 = (SpacetimeDB.StdbVector2)(args_0_value);
				OnStopPlayerEvent(dbEvent.Status, Identity.From(dbEvent.CallerIdentity.ToByteArray()), args_0);
			}
		}
		[DeserializeEvent(FunctionName = "stop_player")]
		public static object[] StopPlayerDeserializeEventArgs(ClientApi.Event dbEvent)
		{
			var bsatnBytes = dbEvent.FunctionCall.ArgBytes;
			using var ms = new System.IO.MemoryStream();
			ms.SetLength(bsatnBytes.Length);
			bsatnBytes.CopyTo(ms.GetBuffer(), 0);
			ms.Position = 0;
			using var reader = new System.IO.BinaryReader(ms);
			var args_0_value = SpacetimeDB.SATS.AlgebraicValue.Deserialize(SpacetimeDB.StdbVector2.GetAlgebraicType(), reader);
			var args_0 = (SpacetimeDB.StdbVector2)(args_0_value);
			return new object[] {
				args_0,
			};
		}
	}
}