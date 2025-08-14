using System;
using System.Linq;
using System.Text.RegularExpressions;

static class LogLine
{
    private static Regex _logRegex = new Regex(@"\[(?<loglevel>\w+?)\]:\s+(?<message>.+)");

    public static string Message(string logLine)
    {
        return GetGroup("message", logLine);
    }

    public static string LogLevel(string logLine)
    {
        return GetGroup("loglevel", logLine).ToLower();
    }

    public static string Reformat(string logLine)
    {
        return $"{Message(logLine)} ({LogLevel(logLine)})";
    }

    private static string GetGroup(string groupName, string logLine)
    {
        MatchCollection matches = _logRegex.Matches(logLine);
        if (matches.Any())
        {
            return matches.First().Groups[groupName].Value.Trim();
        }
        else
        {
            return string.Empty;
        }
    }
}
