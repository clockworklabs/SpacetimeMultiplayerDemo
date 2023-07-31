// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;

namespace SpacetimeDB
{
	public partial class Grass : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("x")]
		public float X;
		[Newtonsoft.Json.JsonProperty("y")]
		public float Y;
		[Newtonsoft.Json.JsonProperty("scale")]
		public float Scale;

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("x", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.F32)),
				new SpacetimeDB.SATS.ProductTypeElement("y", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.F32)),
				new SpacetimeDB.SATS.ProductTypeElement("scale", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.F32)),
			});
		}

		public static explicit operator Grass(SpacetimeDB.SATS.AlgebraicValue value)
		{
			if (value == null) {
				return null;
			}
			var productValue = value.AsProductValue();
			return new Grass
			{
				X = productValue.elements[0].AsF32(),
				Y = productValue.elements[1].AsF32(),
				Scale = productValue.elements[2].AsF32(),
			};
		}

	}
}