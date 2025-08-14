using System;

static class AssemblyLine
{
    public static double SuccessRate(int speed) => speed switch
    {
        0 => 0.0,
        int n when n >= 1 && n <= 4 => 1.0,
        int n when n >= 5 && n <= 8 => 0.9,
        9 => 0.8,
        10 => 0.77,
        _ => throw new ArgumentOutOfRangeException(nameof(speed), "Speed not between 0 and 10, inclusive"),
    };

    public static double ProductionRatePerHour(int speed) => speed * 221 * SuccessRate(speed);

    public static int WorkingItemsPerMinute(int speed) => Convert.ToInt32(Math.Floor(ProductionRatePerHour(speed) / 60.0));
}
