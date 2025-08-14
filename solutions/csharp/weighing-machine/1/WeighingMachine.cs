using System;

class WeighingMachine
{
    public int Precision { get; }

    private double _weight;
    public double Weight
    {
        get
        {
            return _weight;
        }
        set
        {
            if (double.IsNegative(value))
            {
                throw new ArgumentOutOfRangeException(nameof(Weight), "Weight cannot be negative");
            }
            _weight = value;
        }
    }

    public double TareAdjustment { get; set; } = 5.0;

    public string DisplayWeight
    {
        get
        {
            double adjustedWeight = Weight - TareAdjustment;
            string weightString = adjustedWeight.ToString("F" + Precision);
            return $"{weightString} kg";
        }
    }

    public WeighingMachine(int precision)
    {
        Precision = precision;
    }
}
