class RemoteControlCar
{
    private uint _distanceDriven = 0;
    private uint _batteryPercentage = 100;

    public static RemoteControlCar Buy()
    {
        return new RemoteControlCar();
    }

    public string DistanceDisplay()
    {
        return $"Driven {_distanceDriven} meters";
    }

    public string BatteryDisplay()
    {
        if (_batteryPercentage == 0)
        {
            return $"Battery empty";
        }

        return $"Battery at {_batteryPercentage}%";
    }

    public void Drive()
    {
        if (_batteryPercentage > 0)
        {
            _batteryPercentage -= 1;
            _distanceDriven += 20;
        }
    }
}
