class RemoteControlCar
{
    private int _distanceDriven = 0;

    public int BatteryPercentage { get; private set; } = 100;

    public int Speed { get; private set; }
    public int BatteryDrain { get; private set; }

    public RemoteControlCar(int speed, int batteryDrain)
    {
        Speed = speed;
        BatteryDrain = batteryDrain;
    }

    public bool BatteryDrained() => (BatteryPercentage - BatteryDrain) < 0;

    public int DistanceDriven() => _distanceDriven;

    public void Drive()
    {
        if (!BatteryDrained())
        {
            BatteryPercentage -= BatteryDrain;
            _distanceDriven += Speed;
        }
    }

    public static RemoteControlCar Nitro() => new(50, 4);
}

class RaceTrack
{
    private readonly int _distance;

    public RaceTrack(int distance)
    {
        _distance = distance;
    }

    public bool TryFinishTrack(RemoteControlCar car)
    {
        int distanceRemaining = car.BatteryPercentage / car.BatteryDrain * car.Speed;
        return distanceRemaining >= _distance;
    }
}
