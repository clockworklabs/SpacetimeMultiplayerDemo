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
    }