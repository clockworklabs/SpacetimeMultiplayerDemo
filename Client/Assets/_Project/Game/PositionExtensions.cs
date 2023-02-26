    using UnityEngine;

    public static class PositionExtensions
    {
        public static Vector3 ToVector3(this SpacetimeDB.StdbVector3 position)
        {
            return new Vector3
            {
                x = position.X,
                y = position.Y,
                z = position.Z,
            };
        }
        
        public static Quaternion ToQuaternion(this SpacetimeDB.StdbQuaternion rotation)
        {
            return new Quaternion
            {
                x = rotation.X,
                y = rotation.Y,
                z = rotation.Z,
                w = rotation.W,
            };
        }

        public static SpacetimeDB.StdbVector3 ToStdb(this Vector3 vec)
        {
            return new SpacetimeDB.StdbVector3 
            {
                X = vec.x,
                Y = vec.y,
                Z = vec.z,
            };
        }

        public static SpacetimeDB.StdbQuaternion ToStdb(this Quaternion q)
        {
            return new SpacetimeDB.StdbQuaternion 
            {
                X = q.x,
                Y = q.y,
                Z = q.z,
                W = q.w,
            };
        }
    }