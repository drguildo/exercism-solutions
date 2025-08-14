using System;

enum AccountType
{
    Guest,
    User,
    Moderator,
}

[Flags]
enum Permission : byte
{
    None = 0x00,
    Read = 0x01,
    Write = 0x02,
    Delete = 0x04,
    All = Read | Write | Delete,
}


static class Permissions
{
    public static Permission Default(AccountType accountType) => accountType switch
    {
        AccountType.Guest => Permission.Read,
        AccountType.User => Permission.Read | Permission.Write,
        AccountType.Moderator => Permission.All,
        _ => Permission.None,
    };

    public static Permission Grant(Permission current, Permission grant) => current | grant;

    public static Permission Revoke(Permission current, Permission revoke) => current ^ (current & revoke);

    public static bool Check(Permission current, Permission check) => current.HasFlag(check);
}
