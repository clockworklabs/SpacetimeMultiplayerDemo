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

        /// <summary>
        /// Update: I'm going to leave this just in case but likely I think we will never use this. I'm committing
        /// this and then deleting it.
        /// 
        /// Decodes a byte array into a typedef and amount of bytes read.
        /// </summary>
        /// <param name="arr">The array to read from</param>
        /// <param name="offset">The offset to start reading from</param>
        /// <param name="length">The total length of the buffer</param>
        /// <returns>A TypeDef and bytes read value on success, null and 0 on failure</returns>
        public static (TypeDef?, int) Decode(byte[] arr, int offset, int length)
        {
            if (length == 0 || offset >= length - 1)
            {
                Debug.LogError("Array for TypeDef should be greater than 0.");
                return (null, 0);
            }
            
            var bytesRead = 1;
            var enumByte = arr[offset];
            if (!Enum.IsDefined(typeof(Def), enumByte))
            {
                Debug.LogError("Error decoding TypeDef from byte stream (wrong version maybe?).");
                return (null, 0);
            }

            var result = new TypeDef
            {
                type = (Def)enumByte
            };

            switch (result.type)
            {
                case Def.Vec:
                    Debug.LogError("Don't use vecs yet!");
                    return (null, 0);
                case Def.Enum:
                    Debug.LogError("Don't use enums yet!");
                    return (null, 0);
                case Def.Tuple:
                    
                    break;
            }

            return (result, bytesRead);
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
        
        public static (ElementDef?, int) Decode(byte[] arr, int offset, int length)
        {
            var tag = arr[offset];
            var (typeDef, read) = TypeDef.Decode(arr, offset + 1, length);
            if (!typeDef.HasValue)
            {
                return (null, 0);
            }

            return (new ElementDef
            {
                tag = tag,
                element = typeDef.Value,
            }, 1 + read);
        }
    }

    public struct TypeValue
    {
        
    }

    public class TypeDefSerialization
    {
        
    }
}