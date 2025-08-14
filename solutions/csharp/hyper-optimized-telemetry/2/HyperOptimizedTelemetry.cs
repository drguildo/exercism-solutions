using System;
using System.Collections.Generic;

public static class TelemetryBuffer
{
    public static byte[] ToBuffer(long reading)
    {
        var bytes = new List<byte>();

        switch (reading)
        {
            case >= 2_147_483_648 and <= 4_294_967_295:
                bytes.Add(4);
                bytes.AddRange(BitConverter.GetBytes((uint)reading));
                bytes.AddRange(new byte[4]);
                break;
            case >= 65_536 and <= 2_147_483_647:
                bytes.Add(256 - 4);
                bytes.AddRange(BitConverter.GetBytes((int)reading));
                bytes.AddRange(new byte[4]);
                break;
            case >= 0 and <= 65_535:
                bytes.Add(2);
                bytes.AddRange(BitConverter.GetBytes((ushort)reading));
                bytes.AddRange(new byte[6]);
                break;
            case >= -32_768 and <= -1:
                bytes.Add(256 - 2);
                bytes.AddRange(BitConverter.GetBytes((short)reading));
                bytes.AddRange(new byte[6]);
                break;
            case >= -2_147_483_648 and <= -32_769:
                bytes.Add(256 - 4);
                bytes.AddRange(BitConverter.GetBytes((int)reading));
                bytes.AddRange(new byte[4]);
                break;
            default:
                bytes.Add(256 - 8);
                bytes.AddRange(BitConverter.GetBytes(reading));
                break;
        }

        return bytes.ToArray();
    }

    public static long FromBuffer(byte[] buffer)
    {
        if (buffer.Length != 9)
        {
            return 0;
        }

        byte length = buffer[0];
        byte[] bytes = buffer[1..];
        return length switch
        {
            256 - 8 => BitConverter.ToInt64(bytes),
            256 - 4 => BitConverter.ToInt32(bytes),
            256 - 2 => BitConverter.ToInt16(bytes),
            2 => BitConverter.ToUInt16(bytes),
            4 => BitConverter.ToUInt32(bytes),
            _ => 0,
        };
    }
}
