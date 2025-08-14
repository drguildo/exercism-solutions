using System;
using System.Linq;

public static class LogAnalysis
{
    public static string SubstringAfter(this string str, string delimiter) => string.Join("", str.Split(delimiter).Skip(1));

    public static string SubstringBetween(this string str, string start, string end)
    {
        int startIndex = str.IndexOf(start) + start.Length;
        int endIndex = str.IndexOf(end);
        int length = endIndex - startIndex;
        string substring = str.Substring(startIndex, length);
        return substring;
    }

    public static string Message(this string str) => str.SubstringAfter("]: ");

    public static string LogLevel(this string str) => str.SubstringBetween("[", "]");
}