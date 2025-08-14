using System;
using System.Collections.Generic;
using System.Linq;

public interface IRemoteControlCar
{
    int DistanceTravelled { get; }
    int NumberOfVictories { get; }

    void Drive();
}

public class ProductionRemoteControlCar : IComparable<ProductionRemoteControlCar>, IRemoteControlCar
{
    public int DistanceTravelled { get; private set; }
    public int NumberOfVictories { get; set; }

    public int CompareTo(ProductionRemoteControlCar other)
    {
        if (NumberOfVictories < other.NumberOfVictories) { return -1; }
        if (NumberOfVictories > other.NumberOfVictories) { return 1; }
        return 0;
    }

    public void Drive() => DistanceTravelled += 10;
}

public class ExperimentalRemoteControlCar : IRemoteControlCar
{
    public int DistanceTravelled { get; private set; }

    public int NumberOfVictories => 0;

    public void Drive() => DistanceTravelled += 20;
}

public static class TestTrack
{
    public static void Race(IRemoteControlCar car) => car.Drive();

    public static List<ProductionRemoteControlCar> GetRankedCars(ProductionRemoteControlCar prc1,
        ProductionRemoteControlCar prc2)
    {
        var rankings = new List<ProductionRemoteControlCar>() { prc1, prc2 };
        return rankings.OrderBy(c => c.NumberOfVictories).ToList();
    }
}
