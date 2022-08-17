    using SpacetimeDB;
    using UnityEngine;

    public static class PositionExtensions
    {
        public static Vector3 ToVector3(this EntityTransform transform)
        {
            return new Vector3
            {
                x = transform.posX,
                y = transform.posY,
                z = transform.posZ,
            };
        }
        
        public static Quaternion ToQuaternion(this EntityTransform transform)
        {
            return new Quaternion()
            {
                x = transform.rotX,
                y = transform.rotY,
                z = transform.rotZ,
                w = transform.rotW,
            };
        }
    }