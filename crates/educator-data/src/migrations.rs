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
    (
        // Level assessment: the seconds given per question to the assessment (20 by default) and to
        // the exams (0: no timer), chosen in the profile. Existing profiles take the defaults.
        "0003_question_timers",
        r#"
ALTER TABLE "Profile" ADD COLUMN "PlacementSeconds" INTEGER NOT NULL DEFAULT 20;

ALTER TABLE "Profile" ADD COLUMN "ExamSeconds" INTEGER NOT NULL DEFAULT 0;
"#,
    ),
    (
        // Lot 3 (SUB001501 to SUB001513): the avatar of a profile (its character and the accessories
        // it wears, JSON; `CharacterId` keeps the character for the older versions), the badges
        // earned, the accessories unlocked, the days the goal of the day was reached, the review
        // queue of the mistakes and the settings of each child. Existing profiles keep their
        // character, without accessories. Deleting a profile deletes them.
        "0004_motivation",
        r#"
ALTER TABLE "Profile" ADD COLUMN "Avatar" TEXT NOT NULL DEFAULT '';

UPDATE "Profile" SET "Avatar" = json_object('character', "CharacterId", 'accessories', json('[]'));

CREATE TABLE "EarnedBadge" (
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_EarnedBadge_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "BadgeId" TEXT NOT NULL,
    "EarnedAt" INTEGER NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL,
    CONSTRAINT "PK_EarnedBadge" PRIMARY KEY ("ProfileId", "BadgeId")
);

CREATE TABLE "UnlockedAccessory" (
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_UnlockedAccessory_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "AccessoryId" TEXT NOT NULL,
    "UnlockedAt" INTEGER NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL,
    CONSTRAINT "PK_UnlockedAccessory" PRIMARY KEY ("ProfileId", "AccessoryId")
);

CREATE TABLE "DailyGoalDay" (
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_DailyGoalDay_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "Day" INTEGER NOT NULL,
    "ReachedAt" INTEGER NOT NULL,
    "BonusPoints" INTEGER NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL,
    CONSTRAINT "PK_DailyGoalDay" PRIMARY KEY ("ProfileId", "Day")
);

CREATE TABLE "ReviewItem" (
    "Id" TEXT NOT NULL CONSTRAINT "PK_ReviewItem" PRIMARY KEY,
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_ReviewItem_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "SkillId" TEXT NOT NULL,
    "Key" TEXT NOT NULL,
    "Prompt" TEXT NOT NULL,
    "Expected" TEXT NOT NULL,
    "Word" TEXT NULL,
    "OriginSeed" INTEGER NULL,
    "OriginAnswerMode" TEXT NULL,
    "OriginFormat" TEXT NULL,
    "OriginIndex" INTEGER NULL,
    "Step" INTEGER NOT NULL,
    "DueDay" INTEGER NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL
);

CREATE UNIQUE INDEX "IX_ReviewItem_ProfileId_SkillId_Key" ON "ReviewItem" ("ProfileId", "SkillId", "Key");

CREATE TABLE "ProfileSetting" (
    "ProfileId" TEXT NOT NULL CONSTRAINT "FK_ProfileSetting_Profile" REFERENCES "Profile" ("Id") ON DELETE CASCADE,
    "Key" TEXT NOT NULL,
    "Value" TEXT NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL,
    CONSTRAINT "PK_ProfileSetting" PRIMARY KEY ("ProfileId", "Key")
);
"#,
    ),
];
