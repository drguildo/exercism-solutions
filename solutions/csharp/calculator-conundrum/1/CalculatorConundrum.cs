using System;

public static class SimpleCalculator
{
    public static string Calculate(int operand1, int operand2, string operation)
    {
        if (operation == null)
        {
            throw new ArgumentNullException();
        }

        if (operation == "")
        {
            throw new ArgumentException();
        }

        try
        {
            int result = operation switch
            {
                "+" => operand1 + operand2,
                "-" => operand1 - operand2,
                "*" => operand1 * operand2,
                "/" => operand1 / operand2,
                _ => throw new ArgumentOutOfRangeException(nameof(operation), $"Invalid operation"),
            };

            return $"{operand1} {operation} {operand2} = {result}";
        }
        catch (DivideByZeroException)
        {
            return "Division by zero is not allowed.";
        }
    }
}
