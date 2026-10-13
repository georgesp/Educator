//! Versioned migrations, applied in order at start-up. The version of a database is SQLite's
//! `PRAGMA user_version` (number of migrations applied). Never edit a published migration: add one.
//!
//! Migration 1 recreates exactly the schema left by the .NET version (EF Core migrations
//! `InitialCreate` and `AddProfile`) with `IF NOT EXISTS`: a database created by the .NET version
//! (user_version 0) is adopted with its data; its `__EFMigrationsHistory` table is ignored.

/// `(name, SQL)` of each migration, in order.
pub const MIGRATIONS: &[(&str, &str)] = &[
    (
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
    ),
    (
        // Lot 2 (SUB001401, SUB001410): the sessions of the children, their answers, the mastery of
        // each skill and the slide where a lesson was left. Deleting a profile deletes them (foreign
        // keys, enabled on every connection).
        "0002_sessions",
        r#"
CREATE TABLE "Session" (
    "Id" TEXT NOT NULL CONSTRAINT "PK_Session" PRIMARY KEY,
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_Session_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "Type" TEXT NOT NULL,
    "Subject" TEXT NOT NULL,
    "Grade" TEXT NOT NULL,
    "SkillId" TEXT NULL,
    "LessonId" TEXT NULL,
    "AnswerMode" TEXT NULL,
    "Format" TEXT NULL,
    "Seed" INTEGER NOT NULL,
    "QuestionCount" INTEGER NOT NULL,
    "TimeLimitSeconds" INTEGER NULL,
    "StartedAt" INTEGER NOT NULL,
    "EndedAt" INTEGER NULL,
    "Abandoned" INTEGER NOT NULL DEFAULT 0,
    "CorrectCount" INTEGER NOT NULL DEFAULT 0,
    "PointsEarned" INTEGER NOT NULL DEFAULT 0,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL
);

CREATE INDEX "IX_Session_ProfileId_StartedAt" ON "Session" ("ProfileId", "StartedAt");

CREATE TABLE "Answer" (
    "Id" TEXT NOT NULL CONSTRAINT "PK_Answer" PRIMARY KEY,
    "SessionId" TEXT NOT NULL CONSTRAINT "FK_Answer_Session" REFERENCES "Session" ("Id") ON DELETE CASCADE,
    "Index" INTEGER NOT NULL,
    "SkillId" TEXT NOT NULL,
    "Prompt" TEXT NOT NULL,
    "Expected" TEXT NOT NULL,
    "Given" TEXT NOT NULL,
    "IsCorrect" INTEGER NOT NULL,
    "DurationMs" INTEGER NULL,
    "AnsweredAt" INTEGER NOT NULL
);

CREATE UNIQUE INDEX "IX_Answer_SessionId_Index" ON "Answer" ("SessionId", "Index");

CREATE INDEX "IX_Answer_SkillId_AnsweredAt" ON "Answer" ("SkillId", "AnsweredAt");

CREATE TABLE "SkillMastery" (
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_SkillMastery_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "SkillId" TEXT NOT NULL,
    "Score" INTEGER NOT NULL,
    "AttemptCount" INTEGER NOT NULL,
    "LastPracticedAt" INTEGER NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL,
    CONSTRAINT "PK_SkillMastery" PRIMARY KEY ("ProfileId", "SkillId"),
    CONSTRAINT "CK_SkillMastery_Score" CHECK ("Score" BETWEEN 0 AND 100)
);

CREATE TABLE "LessonProgress" (
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_LessonProgress_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "LessonId" TEXT NOT NULL,
    "SlideIndex" INTEGER NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL,
    CONSTRAINT "PK_LessonProgress" PRIMARY KEY ("ProfileId", "LessonId")
);
"#,
    ),
];
