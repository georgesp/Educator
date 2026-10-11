using Educator.Data.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Educator.Data.Configurations;

internal sealed class AppSettingConfiguration : IEntityTypeConfiguration<AppSettingEntity>
{
    public void Configure(EntityTypeBuilder<AppSettingEntity> builder)
    {
        builder.ToTable("AppSetting");
        builder.HasKey(e => e.Key);
    }
}
