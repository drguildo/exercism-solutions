using System.Text;

public static class Identifier
{
    public static string Clean(string identifier)
    {
        if (string.IsNullOrEmpty(identifier))
        {
            return string.Empty;
        }

        var cleanedIdentifier = new StringBuilder();
        bool nextCharToUpper = false;
        foreach (char c in identifier.ToCharArray())
        {
            if (c == ' ')
            {
                cleanedIdentifier.Append('_');
            }
            else if (char.IsControl(c))
            {
                cleanedIdentifier.Append("CTRL");
            }
            else if (c == '-')
            {
                nextCharToUpper = true;
            }
            else if (!char.IsLetter(c) || (c >= 'α' && c <= 'ω'))
            {
                continue;
            }
            else
            {
                if (nextCharToUpper)
                {
                    cleanedIdentifier.Append(char.ToUpper(c));
                    nextCharToUpper = false;
                }
                else
                {
                    cleanedIdentifier.Append(c);
                }
            }
        }

        return cleanedIdentifier.ToString();
    }
}
