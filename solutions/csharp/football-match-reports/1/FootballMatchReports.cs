using System;

public static class PlayAnalyzer
{
    public static string AnalyzeOnField(int shirtNum) => shirtNum switch
    {
        1 => "goalie",
        2 => "left back",
        3 or 4 => "center back",
        5 => "right back",
        6 or 7 or 8 => "midfielder",
        9 => "left wing",
        10 => "striker",
        11 => "right wing",
        _ => throw new ArgumentOutOfRangeException($"Invalid shirt number {shirtNum}"),
    };

    public static string AnalyzeOffField(object report)
    {
        switch (report)
        {
            case int supporters:
                return $"There are {supporters} supporters at the match.";
            case string announcement:
                return announcement;
            case Injury injury:
                return $"Oh no! {injury.GetDescription()} Medics are on the field.";
            case Incident incident:
                return incident.GetDescription();
            case Manager manager:
                if (manager.Club != null)
                {
                    return $"{manager.Name} ({manager.Club})";
                }
                else
                {
                    return $"{manager.Name}";
                }
            default:
                throw new ArgumentException("Unknown type");
        }
    }
}
