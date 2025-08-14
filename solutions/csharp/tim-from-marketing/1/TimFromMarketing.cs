using System.Text;

static class Badge
{
    public static string Print(int? id, string name, string? department)
    {
        var stringBuffer = new StringBuilder();

        if (id != null)
        {
            stringBuffer.Append($"[{id}] - ");
        }

        stringBuffer.Append(name);

        if (department == null)
        {
            stringBuffer.Append(" - OWNER");
        }
        else
        {
            stringBuffer.Append($" - {department.ToUpper()}");
        }

        return stringBuffer.ToString();
    }
}
