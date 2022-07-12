    using SpacetimeDB;
    using UnityEngine;

    public static class PositionExtensions
    {
        public static SpacetimeDB.Position ToStdb(this Vector3 position)
        {
            return new Position
            {
                posX = position.x,
                posY = position.y,
                posZ = position.z,
            };
        }
        
        public static Vector3 ToVector3(this SpacetimeDB.Position position)
        {
            return new Vector3
            {
                x = position.posX,
                y = position.posY,
                z = position.posZ,
            };
        }
    }