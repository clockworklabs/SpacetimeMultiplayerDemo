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
        
        public static SpacetimeDB.Rotation ToStdb(this Quaternion rotation)
        {
            return new SpacetimeDB.Rotation
            {
                rotX = rotation.x,
                rotY = rotation.y,
                rotZ = rotation.z,
                rotW = rotation.w,
            };
        }
        
        public static Quaternion ToQuaternion(this SpacetimeDB.Rotation rotation)
        {
            return new Quaternion()
            {
                x = rotation.rotX,
                y = rotation.rotY,
                z = rotation.rotZ,
                w = rotation.rotW,
            };
        }
    }