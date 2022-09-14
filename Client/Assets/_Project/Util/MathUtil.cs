using System;
using UnityEngine;

public static class MathUtil
{
    public static int RoundNegInf(double val)
    {
        var intValue = (int)val;
        if (Mathf.Approximately((float)(val - intValue), 0.0f))
        {
            return Mathf.RoundToInt((float)val);
        }

        return val >= 0 ? Mathf.FloorToInt((float)val) : Mathf.FloorToInt((float)(val + 1));
    }
    
    public static int RoundNegInf(float val)
    {
        var intValue = (int)val;
        if (Mathf.Approximately((val - intValue), 0.0f))
        {
            return Mathf.RoundToInt(val);
        }

        return val >= 0 ? Mathf.FloorToInt(val) : Mathf.FloorToInt(val + 1);
    }
}