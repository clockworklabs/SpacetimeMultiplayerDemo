using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using UnityEditor.Rendering;
using Debug = UnityEngine.Debug;

namespace SpacetimeDB
{
    public struct TypeDef
    {
        public enum Def
        {
            Tuple = 0,
            Enum,

            // base types
            Vec,
            U8,
            U16,
            U32,
            U64,
            U128,
            I8,
            I16,
            I32,
            I64,
            I128,
            Bool,
            F32,
            F64,
            String,
            Bytes,
            Unit,
        }

        public Def Type => type;
        public Def? VecMemberType => vecMemberType;
        public ElementDef[] TupleElements => tupleElements;

        private Def type;
        private Def? vecMemberType;
        private ElementDef[] tupleElements;

        public static TypeDef GetVec(Def memberType)
        {
            return new TypeDef
            {
                type = Def.Vec,
                vecMemberType = memberType,
            };
        }

        public static TypeDef BuiltInType(Def def)
        {
            switch (def)
            {
                case Def.Tuple:
                case Def.Enum:
                case Def.Vec:
                    Debug.LogError("This is not a built-in type");
                    break;
            }

            return new TypeDef
            {
                type = def,
            };
        }
        
        public static TypeDef Tuple(ElementDef[] elements)
        {
            return new TypeDef
            {
                type = Def.Tuple,
                tupleElements = elements,
            };
        }
    }

    public struct ElementDef
    {
        public byte tag;
        public TypeDef element;

        public ElementDef(byte tag, TypeDef element)
        {
            this.tag = tag;
            this.element = element;
        }
    }

    public struct TypeValue
    {
        
    }

    public class TypeDefSerialization
    {
        
    }
}