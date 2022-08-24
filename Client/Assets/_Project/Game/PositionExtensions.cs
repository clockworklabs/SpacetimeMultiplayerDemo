    using SpacetimeDB;
    using UnityEngine;

    public static class PositionExtensions
    {
        public static Vector3 ToVector3(this StdbPosition position)
        {
            return new Vector3
            {
                x = position.x,
                y = position.y,
                z = position.z,
            };
        }
        
        public static Quaternion ToQuaternion(this StdbQuaternion rotation)
        {
            return new Quaternion()
            {
                x = rotation.x,
                y = rotation.y,
                z = rotation.z,
                w = rotation.w,
            };
        }

        public static StdbPosition ToStdb(this Vector3 vec)
        {
            return new StdbPosition
            {
                x = vec.x,
                y = vec.y,
                z = vec.z,
            };
        }

        public static StdbQuaternion ToStdb(this Quaternion q)
        {
            return new StdbQuaternion
            {
                x = q.x,
                y = q.y,
                z = q.z,
                w = q.w,
            };
        }
    }