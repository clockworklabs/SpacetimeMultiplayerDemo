// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

namespace SpacetimeDB
{
	public partial class Rotation
	{
		[Newtonsoft.Json.JsonProperty("rot_x")]
		public float rotX;
		[Newtonsoft.Json.JsonProperty("rot_y")]
		public float rotY;
		[Newtonsoft.Json.JsonProperty("rot_z")]
		public float rotZ;
		[Newtonsoft.Json.JsonProperty("rot_w")]
		public float rotW;
		public static TypeDef GetTypeDef()
		{
			return TypeDef.Tuple(new ElementDef[]
			{
				new SpacetimeDB.ElementDef(0, SpacetimeDB.TypeDef.BuiltInType(SpacetimeDB.TypeDef.Def.F32)),
				new SpacetimeDB.ElementDef(1, SpacetimeDB.TypeDef.BuiltInType(SpacetimeDB.TypeDef.Def.F32)),
				new SpacetimeDB.ElementDef(2, SpacetimeDB.TypeDef.BuiltInType(SpacetimeDB.TypeDef.Def.F32)),
				new SpacetimeDB.ElementDef(3, SpacetimeDB.TypeDef.BuiltInType(SpacetimeDB.TypeDef.Def.F32)),
			});
		}
	}
}