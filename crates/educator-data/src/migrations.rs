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
    (
        // Lot tablette 2 (US0276, ADR 0007): the synchronisation between the computer and the iPad.
        // `SyncState`: identity of this device and values of the synchronisation; `SyncRow`: the
        // clock and sequence number (`SyncCounter`) of every synchronised record (one row per field of a profile,
        // `Deleted` for the tombstones); `SyncPeer`: the paired devices. Triggers follow every
        // write. The points are recomputed from the sessions.
        "0005_sync",
        r#"
-- Points are the points of the closed sessions and the bonuses of the goal of the day (ADR 0007,
-- section 6): those of the .NET version, matching no session, are dropped (Georges, 2026-10-15).
UPDATE "Profile" SET "Points" =
    (SELECT COALESCE(SUM("PointsEarned"), 0) FROM "Session" WHERE "ProfileId" = "Profile"."Id" AND "EndedAt" IS NOT NULL)
    + (SELECT COALESCE(SUM("BonusPoints"), 0) FROM "DailyGoalDay" WHERE "ProfileId" = "Profile"."Id");

CREATE TABLE "SyncState" (
    "Key" TEXT NOT NULL CONSTRAINT "PK_SyncState" PRIMARY KEY,
    "Value" TEXT NOT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL
);

CREATE TABLE "SyncRow" (
    "Entity" TEXT NOT NULL,
    "Key" TEXT NOT NULL,
    "Field" TEXT NOT NULL DEFAULT '',
    "Hlc" TEXT NOT NULL,
    "Seq" INTEGER NOT NULL,
    "Deleted" INTEGER NOT NULL DEFAULT 0,
    "Target" TEXT NULL,
    CONSTRAINT "PK_SyncRow" PRIMARY KEY ("Entity", "Key", "Field")
);

CREATE UNIQUE INDEX "IX_SyncRow_Seq" ON "SyncRow" ("Seq");

-- The last sequence number given: it only grows, even when rows of SyncRow are deleted.
CREATE TABLE "SyncCounter" ("Seq" INTEGER NOT NULL);

INSERT INTO "SyncCounter" ("Seq") VALUES (0);

CREATE TABLE "SyncPeer" (
    "DeviceId" TEXT NOT NULL CONSTRAINT "PK_SyncPeer" PRIMARY KEY,
    "Role" TEXT NOT NULL,
    "Name" TEXT NOT NULL,
    "TokenHash" TEXT NULL,
    "Token" TEXT NULL,
    "Fingerprint" TEXT NULL,
    "Addresses" TEXT NOT NULL DEFAULT '',
    "Port" INTEGER NULL,
    "PairedAt" INTEGER NOT NULL,
    "LastSyncAt" INTEGER NULL,
    "PushedSeq" INTEGER NOT NULL DEFAULT 0,
    "PulledSeq" INTEGER NOT NULL DEFAULT 0,
    "Revoked" INTEGER NOT NULL DEFAULT 0,
    "Epoch" TEXT NULL,
    "CreatedAt" INTEGER NOT NULL,
    "UpdatedAt" INTEGER NOT NULL
);

-- Every write of a followed table records its change in the same transaction: a clock
-- (educator_hlc(), registered by educator-data on the connection) and a local sequence number.
CREATE TRIGGER "TR_Profile_Sync_Insert" AFTER INSERT ON "Profile"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'name', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'grade', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'avatar', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'placementSeconds', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'examSeconds', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'lastUsedAt', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
    DELETE FROM "SyncRow" WHERE "Entity" = 'Profile' AND "Key" = NEW."Id" AND "Field" = '';
END;

CREATE TRIGGER "TR_Profile_Sync_Name" AFTER UPDATE OF "Name" ON "Profile"
WHEN OLD."Name" IS NOT NEW."Name"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'name', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Profile_Sync_Grade" AFTER UPDATE OF "Grade" ON "Profile"
WHEN OLD."Grade" IS NOT NEW."Grade"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'grade', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Profile_Sync_PlacementSeconds" AFTER UPDATE OF "PlacementSeconds" ON "Profile"
WHEN OLD."PlacementSeconds" IS NOT NEW."PlacementSeconds"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'placementSeconds', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Profile_Sync_ExamSeconds" AFTER UPDATE OF "ExamSeconds" ON "Profile"
WHEN OLD."ExamSeconds" IS NOT NEW."ExamSeconds"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'examSeconds', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Profile_Sync_LastUsedAt" AFTER UPDATE OF "LastUsedAt" ON "Profile"
WHEN OLD."LastUsedAt" IS NOT NEW."LastUsedAt"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'lastUsedAt', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Profile_Sync_Avatar" AFTER UPDATE OF "CharacterId", "Avatar" ON "Profile"
WHEN OLD."CharacterId" IS NOT NEW."CharacterId" OR OLD."Avatar" IS NOT NEW."Avatar"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', NEW."Id", 'avatar', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Profile_Sync_Delete" AFTER DELETE ON "Profile"
BEGIN
    DELETE FROM "SyncRow" WHERE "Entity" = 'Profile' AND "Key" = OLD."Id" AND "Field" <> '';
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Profile', OLD."Id", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 1, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Session_Sync_Insert" AFTER INSERT ON "Session"
WHEN NEW."EndedAt" IS NOT NULL
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Session', NEW."Id", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Session_Sync_Update" AFTER UPDATE ON "Session"
WHEN NEW."EndedAt" IS NOT NULL AND (OLD."EndedAt" IS NULL OR OLD."ProfileId" IS NOT NEW."ProfileId")
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Session', NEW."Id", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_Session_Sync_Delete" AFTER DELETE ON "Session"
BEGIN
    DELETE FROM "SyncRow" WHERE "Entity" = 'Session' AND "Key" = OLD."Id";
END;

CREATE TRIGGER "TR_LessonProgress_Sync_Insert" AFTER INSERT ON "LessonProgress"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('LessonProgress', NEW."ProfileId" || '/' || NEW."LessonId", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_LessonProgress_Sync_Update" AFTER UPDATE ON "LessonProgress"
WHEN OLD."SlideIndex" IS NOT NEW."SlideIndex" OR OLD."ProfileId" IS NOT NEW."ProfileId"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('LessonProgress', NEW."ProfileId" || '/' || NEW."LessonId", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_LessonProgress_Sync_Delete" AFTER DELETE ON "LessonProgress"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('LessonProgress', OLD."ProfileId" || '/' || OLD."LessonId", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 1, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_EarnedBadge_Sync_Insert" AFTER INSERT ON "EarnedBadge"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Badge', NEW."ProfileId" || '/' || NEW."BadgeId", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_EarnedBadge_Sync_Update" AFTER UPDATE ON "EarnedBadge"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Badge', NEW."ProfileId" || '/' || NEW."BadgeId", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_EarnedBadge_Sync_Delete" AFTER DELETE ON "EarnedBadge"
BEGIN
    DELETE FROM "SyncRow" WHERE "Entity" = 'Badge' AND "Key" = OLD."ProfileId" || '/' || OLD."BadgeId";
END;

CREATE TRIGGER "TR_UnlockedAccessory_Sync_Insert" AFTER INSERT ON "UnlockedAccessory"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Accessory', NEW."ProfileId" || '/' || NEW."AccessoryId", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_UnlockedAccessory_Sync_Update" AFTER UPDATE ON "UnlockedAccessory"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Accessory', NEW."ProfileId" || '/' || NEW."AccessoryId", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_UnlockedAccessory_Sync_Delete" AFTER DELETE ON "UnlockedAccessory"
BEGIN
    DELETE FROM "SyncRow" WHERE "Entity" = 'Accessory' AND "Key" = OLD."ProfileId" || '/' || OLD."AccessoryId";
END;

CREATE TRIGGER "TR_DailyGoalDay_Sync_Insert" AFTER INSERT ON "DailyGoalDay"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('GoalDay', NEW."ProfileId" || '/' || NEW."Day", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_DailyGoalDay_Sync_Update" AFTER UPDATE ON "DailyGoalDay"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('GoalDay', NEW."ProfileId" || '/' || NEW."Day", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_DailyGoalDay_Sync_Delete" AFTER DELETE ON "DailyGoalDay"
BEGIN
    DELETE FROM "SyncRow" WHERE "Entity" = 'GoalDay' AND "Key" = OLD."ProfileId" || '/' || OLD."Day";
END;

CREATE TRIGGER "TR_ReviewItem_Sync_Insert" AFTER INSERT ON "ReviewItem"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Review', NEW."ProfileId" || '/' || NEW."SkillId" || '/' || NEW."Key", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_ReviewItem_Sync_Update" AFTER UPDATE ON "ReviewItem"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Review', NEW."ProfileId" || '/' || NEW."SkillId" || '/' || NEW."Key", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_ReviewItem_Sync_Delete" AFTER DELETE ON "ReviewItem"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Review', OLD."ProfileId" || '/' || OLD."SkillId" || '/' || OLD."Key", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 1, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_ProfileSetting_Sync_Insert" AFTER INSERT ON "ProfileSetting"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Setting', NEW."ProfileId" || '/' || NEW."Key", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_ProfileSetting_Sync_Update" AFTER UPDATE ON "ProfileSetting"
WHEN OLD."Value" IS NOT NEW."Value" OR OLD."ProfileId" IS NOT NEW."ProfileId"
BEGIN
    UPDATE "SyncCounter" SET "Seq" = "Seq" + 1;
    INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
    VALUES ('Setting', NEW."ProfileId" || '/' || NEW."Key", '', educator_hlc(), (SELECT "Seq" FROM "SyncCounter"), 0, NULL)
    ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
        "Deleted" = excluded."Deleted", "Target" = NULL;
END;

CREATE TRIGGER "TR_ProfileSetting_Sync_Delete" AFTER DELETE ON "ProfileSetting"
BEGIN
    DELETE FROM "SyncRow" WHERE "Entity" = 'Setting' AND "Key" = OLD."ProfileId" || '/' || OLD."Key";
END;
"#,
    ),
];
