#ifndef HEX_COORDINATES_UTIL_INCLUDED
#define HEX_COORDINATES_UTIL_INCLUDED

#include "Conditionals.hlsl"

static float outerRadius = 10;
static float innerRadius = 8.66025404;

static float2 neighborOffsets[6] = {
    float2(0,1),
    float2(1,0),
    float2(1,-1),
    float2(0,-1),
    float2(-1,0),
    float2(-1,1)
};

static float2 vertices[6] = {
    float2(0, outerRadius),
    float2(innerRadius, outerRadius / 2),
    float2(innerRadius, -outerRadius / 2),
    float2(0, -outerRadius),
    float2(-innerRadius, -outerRadius / 2),
    float2(-innerRadius, outerRadius / 2)
};

float2 toOffset(float2 hexCoordinates) {
    return float2(hexCoordinates.x + trunc(hexCoordinates.y / 2), hexCoordinates.y);
}

float2 positionToCoordinates(float3 position) {
    float x = position.x / (innerRadius * 2);
    float y = -x;
    float offset = position.z / (outerRadius * 3);
    x -= offset;
    y -= offset;
    int iX = round(x);
    int iY = round(y);
    int iZ = round(-x - y);

    if (iX + iY + iZ != 0) {
        float dX = abs(x - iX);
        float dY = abs(y - iY);
        float dZ = abs(-x - y - iZ);

        if (dX > dY && dX > dZ) {
            iX = -iY - iZ;
        }
        else if (dZ > dY) {
            iZ = -iX - iY;
        }
    }
    return float2(iX, iZ);
}

float2 coordinatesToPosition(float2 coordinates) {
    float x = 2 * coordinates.x * innerRadius + coordinates.y * innerRadius;
    float z = 1.5 * coordinates.y * outerRadius;
    return float2(x, z);
}

static int directionOfPoint(float2 p) {
    return floor((6 - (atan2(p.y, p.x) / PI + 1) * 3) + 4.5) % 6;
}

static float distanceFromBorder(float2 p0, float2 coordinates) {
    float2 center = coordinatesToPosition(coordinates);
    float minDistance = 1000;
    for (int i = 1; i <= 6; ++i) {
        float2 p1 = center + vertices[i-1];
        float2 p2 = center + vertices[i % 6];
        float2 segment = p2 - p1;
        float r = dot(segment, p0 - p1) / pow(length(segment), 2);
        float dist;
        if (r < 0) {
            dist = length(p0 - p1);
        } else if (r > 1) {
            dist = length(p0 - p2);
        } else {
            dist = sqrt(pow(length(p0 - p1), 2) - pow(r * length(p2 - p1), 2));
        }
        //float dist = when_lt(r, 0) * length(p0 - p1) + 
        //             when_gt(r, 1) * length(p0 - p2);
        //dist = when_eq(dist, 0) * sqrt(pow(length(p0 - p1), 2) - pow(r * length(p2 - p1), 2));
        minDistance = min(dist, minDistance);
    }
    return minDistance;
}

void interpolatedWeights(float3 p, out float3 coordinatesW, out float3 neighbor1W, out float3 neighbor2W) {
    float2 groundPosition = float2(p.x, p.z);
    coordinatesW.xy = positionToCoordinates(p);

    // triangle vertices for barycentric interpolation
    float2 v1 = coordinatesToPosition(coordinatesW.xy);
    float2 v2;
    float2 v3;

    neighbor1W = float3(0,0,0);
    neighbor2W = float3(0,0,0);
    float min1 = 100;
    float min2 = 100;
    for (int i = 0; i != 6; ++i) {
        float2 neighbor = coordinatesW.xy + neighborOffsets[i];
        float2 ncenter = coordinatesToPosition(neighbor);
        float2 offset = ncenter - groundPosition;
        float d = length(offset);

        if (d < min1) {
            // update min2
            neighbor2W = neighbor1W;
            v3 = v2;
            min2 = min1;

            neighbor1W.xy = neighbor;
            v2 = ncenter;
            min1 = d;
        } else if (d < min2) {
            neighbor2W.xy = neighbor;
            v3 = ncenter;
            min2 = d;
        }
    }

    //barycentric interpolation
    coordinatesW.z = ((v2.y-v3.y)*(p.x-v3.x)+(v3.x-v2.x)*(p.z-v3.y))/((v2.y-v3.y)*(v1.x-v3.x)+(v3.x-v2.x)*(v1.y-v3.y));
    neighbor1W.z = ((v3.y-v1.y)*(p.x-v3.x)+(v1.x-v3.x)*(p.z-v3.y))/((v2.y-v3.y)*(v1.x-v3.x)+(v3.x-v2.x)*(v1.y-v3.y));
    neighbor2W.z = 1 - coordinatesW.z - neighbor1W.z;
}

#endif