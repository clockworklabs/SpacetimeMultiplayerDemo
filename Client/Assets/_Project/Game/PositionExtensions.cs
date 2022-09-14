    using UnityEngine;

    public static class PositionExtensions
    {
        public static Vector3 ToVector3(this SpacetimeDB.StdbVector3 position)
        {
            return new Vector3
            {
                x = position.x,
                y = position.y,
                z = position.z,
            };
        }
        
        public static Quaternion ToQuaternion(this SpacetimeDB.StdbQuarternion rotation)
        {
            return new Quaternion
            {
                x = rotation.x,
                y = rotation.y,
                z = rotation.z,
                w = rotation.w,
            };
        }

        public static SpacetimeDB.StdbVector3 ToStdb(this Vector3 vec)
        {
            return new SpacetimeDB.StdbVector3 
            {
                x = vec.x,
                y = vec.y,
                z = vec.z,
            };
        }

        public static SpacetimeDB.StdbQuarternion ToStdb(this Quaternion q)
        {
            return new SpacetimeDB.StdbQuarternion 
            {
                x = q.x,
                y = q.y,
                z = q.z,
                w = q.w,
            };
        }
    }