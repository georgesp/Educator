//! Versioned migrations, applied in order at start-up. The version of a database is SQLite's
//! `PRAGMA user_version` (number of migrations applied). Never edit a published migration: add one.
//!
//! Migration 1 recreates exactly the schema left by the .NET version (EF Core migrations
//! `InitialCreate` and `AddProfile`) with `IF NOT EXISTS`: a database created by the .NET version
//! (user_version 0) is adopted with its data; its `__EFMigrationsHistory` table is ignored.

/// `(name, SQL)` of each migration, in order.
pub const MIGRATIONS: &[(&str, &str)] = &[(
    "0001_initial",
    r#"
CREATE TABLE IF NOT EXISTS "AppSetting" (
    "Key" TEXT NOT NULL CONSTRAINT "PK_AppSetting" PRIMARY KEY,
    "Value" TEXT NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS "Profile" (
    "Id" TEXT NOT NULL CONSTRAINT "PK_Profile" PRIMARY KEY,
    "Name" TEXT NOT NULL,
    "NormalizedName" TEXT NOT NULL,
    "Grade" TEXT NOT NULL,
    "CharacterId" TEXT NOT NULL,
    "Points" INTEGER NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL,
    "LastUsedAt" INTEGER NOT NULL,
    CONSTRAINT "CK_Profile_Points" CHECK ("Points" >= 0)
);

CREATE INDEX IF NOT EXISTS "IX_Profile_LastUsedAt" ON "Profile" ("LastUsedAt");

CREATE UNIQUE INDEX IF NOT EXISTS "IX_Profile_NormalizedName" ON "Profile" ("NormalizedName");
"#,
)];
