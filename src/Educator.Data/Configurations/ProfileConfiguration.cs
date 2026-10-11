using Educator.Core.Profiles;
using Educator.Data.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Educator.Data.Configurations;

internal sealed class ProfileConfiguration : IEntityTypeConfiguration<ProfileEntity>
{
    public void Configure(EntityTypeBuilder<ProfileEntity> builder)
    {
        builder.ToTable("Profile", table => table.HasCheckConstraint("CK_Profile_Points", "\"Points\" >= 0"));
        builder.HasKey(e => e.Id);
        builder.Property(e => e.Id).ValueGeneratedNever();
        builder.Property(e => e.Name).HasMaxLength(Profile.MaxNameLength);
        builder.Property(e => e.NormalizedName).HasMaxLength(Profile.MaxNameLength);
        builder.HasIndex(e => e.NormalizedName).IsUnique();
        builder.Property(e => e.CharacterId).HasMaxLength(64);
        builder.HasIndex(e => e.LastUsedAt);
    }
}
