abstract class Character
{
    private readonly string _characterType;

    protected Character(string characterType) => _characterType = characterType;

    public abstract int DamagePoints(Character target);

    public virtual bool Vulnerable() => false;

    public override string ToString() => $"Character is a {_characterType}";
}

class Warrior : Character
{
    public Warrior() : base(nameof(Warrior))
    {
    }

    public override int DamagePoints(Character target)
    {
        if (target.Vulnerable())
        {
            return 10;
        }

        return 6;
    }
}

class Wizard : Character
{
    private bool _preparedSpell;

    public Wizard() : base(nameof(Wizard)) => _preparedSpell = false;

    public override int DamagePoints(Character target)
    {
        if (_preparedSpell)
        {
            return 12;
        }

        return 3;
    }

    public void PrepareSpell() => _preparedSpell = true;

    public override bool Vulnerable() => !_preparedSpell;
}
