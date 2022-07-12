using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Text;
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
        public TypeDef TypeDef => typeDef;
        
        private TypeDef typeDef;
        private ulong unsigned;
        private long signed;
        private string str;
        private byte[] bytes;
        private bool b;

        private float f32;
        private double f64;
        private TypeValue[] tupleElements;

        public static (TypeValue?, int) Decode(TypeDef def, byte[] arr, int offset, int length)
        {
            var value = new TypeValue
            {
                typeDef = def
            };
            var read = 0;
            
            switch (def.Type)
            {
               case TypeDef.Def.Bool:
                   value.b = arr[offset] != 0;
                   read = 1;
                   break;
               case TypeDef.Def.U8:
                   value.unsigned = arr[offset];
                   read = 1;
                   break;
               case TypeDef.Def.U16:
                   value.unsigned = BitConverter.ToUInt16(arr, offset);
                   read = 2;
                   break;
               case TypeDef.Def.U32:
                   value.unsigned = BitConverter.ToUInt32(arr, offset);
                   read = 4;
                   break;
               case TypeDef.Def.U64:
                   value.unsigned = BitConverter.ToUInt64(arr, offset);
                   read = 8;
                   break;
               case TypeDef.Def.I8:
                   value.signed = arr[offset];
                   read = 1;
                   break;
               case TypeDef.Def.I16:
                   value.signed = BitConverter.ToInt16(arr, offset);
                   read = 2;
                   break;
               case TypeDef.Def.I32:
                   value.signed = BitConverter.ToInt32(arr, offset);
                   read = 4;
                   break;
               case TypeDef.Def.I64:
                   value.signed = BitConverter.ToInt64(arr, offset);
                   read = 8;
                   break;
               case TypeDef.Def.F32:
                   value.f32 = BitConverter.ToSingle(arr, offset);
                   read = 4;
                   break;
               case TypeDef.Def.F64:
                   value.f64 = BitConverter.ToDouble(arr, offset);
                   read = 8;
                   break;
               case TypeDef.Def.String:
                   var strLength = BitConverter.ToUInt16(arr, offset);
                   value.str = Encoding.UTF8.GetString(arr, offset + 2, strLength);
                   read += strLength + 2;
                   break;
               case TypeDef.Def.Bytes:
                   var byteLength = BitConverter.ToUInt16(arr, offset);
                   value.bytes = new byte[byteLength];
                   Array.Copy(arr, offset + 2, value.bytes, 0, byteLength);
                   read += byteLength + 2;
                   break;
               case TypeDef.Def.Tuple:
                   return ReadTuple(def, arr, offset, length);
               default:
                   Debug.LogError($"This type is unsupported for now: {def.Type}");
                   return (null, 0);
            }

            return (value, read);
        }

        private static (TypeValue?, int) ReadTuple(TypeDef def, byte[] arr, int offset, int length)
        {
            var read = 0;
            var resultElements = new TypeValue[def.TupleElements.Length];
            var elementIdx = 0;
            foreach (var elementDef in def.TupleElements)
            {
                var (value, bytesRead) = Decode(elementDef.element, arr, offset + read, length);

                if (!value.HasValue)
                {
                    return (null, 0);
                }

                read += bytesRead;
                resultElements[elementIdx++] = value.Value;
            }

            return (GetTuple(def, resultElements), read);
        }
        
        public object GetValue(TypeDef.Def def)
        {
            switch (def)
            {
                case TypeDef.Def.Bool:
                    return b;
                case TypeDef.Def.U8:
                    return (byte)unsigned;
                case TypeDef.Def.U16:
                    return (ushort)unsigned;
                case TypeDef.Def.U32:
                    return (uint)unsigned;
                case TypeDef.Def.U64:
                    return (uint)unsigned;
                case TypeDef.Def.U128:
                    throw new InvalidOperationException("U128 not supported in C#");
                
                case TypeDef.Def.I8:
                    return (byte)signed;
                case TypeDef.Def.I16:
                    return (ushort)signed;
                case TypeDef.Def.I32:
                    return (uint)signed;
                case TypeDef.Def.I64:
                    return (uint)signed;
                case TypeDef.Def.I128:
                    throw new InvalidOperationException("I128 not supported in C#");
                
                case TypeDef.Def.String:
                    return str;
                case TypeDef.Def.Bytes:
                    return bytes;
                
                case TypeDef.Def.F32:
                    return f32;
                case TypeDef.Def.F64:
                    return f64;
                case TypeDef.Def.Tuple:
                    return tupleElements;
            }
            
            throw new InvalidOperationException($"Type not supported yet! {def}");
        }

        public static TypeValue GetTuple(TypeDef def, TypeValue[] tupleValues)
        {
            return new TypeValue
            {
                typeDef = def,
                tupleElements = tupleValues,
            };
        }
    }

    public class TypeDefSerialization
    {
        
    }
}