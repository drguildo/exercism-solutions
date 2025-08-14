using System;
using System.Globalization;
using System.Reflection.Metadata.Ecma335;
using System.Runtime.InteropServices;

public enum Location
{
    NewYork,
    London,
    Paris
}

public enum AlertLevel
{
    Early,
    Standard,
    Late
}

public static class Appointment
{
    private static TimeZoneInfo LocationToTimeZoneInfo(Location location)
    {
        string newYorkTimeZoneId;
        string londonTimeZoneId;
        string parisTimeZoneId;
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
        {
            newYorkTimeZoneId = "Eastern Standard Time";
            londonTimeZoneId = "GMT Standard Time";
            parisTimeZoneId = "W. Europe Standard Time";
        }
        else
        {
            newYorkTimeZoneId = "America/New_York";
            londonTimeZoneId = "Europe/London";
            parisTimeZoneId = "Europe/Paris";
        }

        return location switch
        {
            Location.NewYork => TimeZoneInfo.FindSystemTimeZoneById(newYorkTimeZoneId),
            Location.London => TimeZoneInfo.FindSystemTimeZoneById(londonTimeZoneId),
            Location.Paris => TimeZoneInfo.FindSystemTimeZoneById(parisTimeZoneId),
            _ => throw new InvalidOperationException("Unsupported location"),
        };
    }

    public static DateTime ShowLocalTime(DateTime dtUtc) => dtUtc.ToLocalTime();

    public static DateTime Schedule(string appointmentDateDescription, Location location)
    {
        DateTime dateTime = DateTime.Parse(appointmentDateDescription);
        TimeZoneInfo sourceTimeZoneInfo = LocationToTimeZoneInfo(location);

        return TimeZoneInfo.ConvertTime(dateTime, sourceTimeZoneInfo, TimeZoneInfo.Utc);
    }

    public static DateTime GetAlertTime(DateTime appointment, AlertLevel alertLevel) => alertLevel switch
    {
        AlertLevel.Early => appointment.AddDays(-1),
        AlertLevel.Standard => appointment.AddMinutes(-105),
        AlertLevel.Late => appointment.AddMinutes(-30),
        _ => throw new NotImplementedException(),
    };

    public static bool HasDaylightSavingChanged(DateTime dt, Location location)
    {
        TimeZoneInfo timeZoneInfo = LocationToTimeZoneInfo(location);
        DateTime dtPrevious = dt.AddDays(-7);
        return timeZoneInfo.IsDaylightSavingTime(dt) != timeZoneInfo.IsDaylightSavingTime(dtPrevious);
    }

    public static DateTime NormalizeDateTime(string dtStr, Location location)
    {
        CultureInfo cultureInfo = location switch
        {
            Location.NewYork => new CultureInfo("en-US"),
            Location.London => new CultureInfo("en-GB"),
            Location.Paris => new CultureInfo("fr-FR"),
            _ => throw new InvalidOperationException("Unsupported location"),
        };

        if (DateTime.TryParse(dtStr, cultureInfo, out DateTime normalizedDateTime))
        {
            return normalizedDateTime;
        }
        else
        {
            return new DateTime(1, 1, 1);
        }
    }
}
