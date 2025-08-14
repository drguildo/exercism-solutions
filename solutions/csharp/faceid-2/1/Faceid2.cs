using System;
using System.Collections.Generic;

public class FacialFeatures(string eyeColor, decimal philtrumWidth)
{
    public string EyeColor { get; } = eyeColor;
    public decimal PhiltrumWidth { get; } = philtrumWidth;

    public override bool Equals(object obj)
    {
        if (obj == null || GetType() != obj.GetType())
        {
            return false;
        }

        FacialFeatures other = (FacialFeatures)obj;

        return EyeColor == other.EyeColor && PhiltrumWidth == other.PhiltrumWidth;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(EyeColor, PhiltrumWidth);
    }
}

public class Identity(string email, FacialFeatures facialFeatures)
{
    public string Email { get; } = email;
    public FacialFeatures FacialFeatures { get; } = facialFeatures;

    public override bool Equals(object obj)
    {
        if (obj == null || GetType() != obj.GetType())
        {
            return false;
        }

        Identity other = (Identity)obj;

        return string.Equals(Email, other.Email) && FacialFeatures.Equals(other.FacialFeatures);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Email, FacialFeatures);
    }
}

public class Authenticator
{
    private readonly Dictionary<string, Identity> _identities = [];

    public static bool AreSameFace(FacialFeatures faceA, FacialFeatures faceB)
    {
        return faceA.Equals(faceB);
    }

    public static bool IsAdmin(Identity identity)
    {
        return identity.Equals(new Identity("admin@exerc.ism", new FacialFeatures("green", 0.9m)));
    }

    public bool Register(Identity identity)
    {
        if (_identities.ContainsKey(identity.Email))
        {
            return false;
        }

        _identities.Add(identity.Email, identity);

        return true;
    }

    public bool IsRegistered(Identity identity)
    {
        if (_identities.TryGetValue(identity.Email, out Identity existingIdentity))
        {
            return existingIdentity.Equals(identity);
        }

        return false;
    }

    public static bool AreSameObject(Identity identityA, Identity identityB)
    {
        return identityA == identityB;
    }
}
