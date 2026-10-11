namespace Educator.Host.Tests;

[Trait("Category", "Unit")]
public sealed class HostCommandLineTests
{
    [Fact]
    public void Parse_NoArgument_UsesAFreePort()
    {
        var commandLine = HostCommandLine.Parse([]);

        Assert.Equal(0, commandLine.Port);
        Assert.Null(commandLine.ParentProcessId);
        Assert.Null(commandLine.ExportOpenApiPath);
    }

    [Fact]
    public void Parse_AllOptions()
    {
        var commandLine = HostCommandLine.Parse(["--port", "5250", "--parent-pid", "1234", "--export-openapi", "out.json"]);

        Assert.Equal(new HostCommandLine(5250, 1234, "out.json"), commandLine);
    }

    [Theory]
    [InlineData("--port")]
    [InlineData("--port", "abc")]
    [InlineData("--port", "70000")]
    [InlineData("--parent-pid", "0")]
    [InlineData("--export-openapi")]
    [InlineData("--unknown")]
    public void Parse_InvalidArguments_Throws(params string[] args)
        => Assert.Throws<ArgumentException>(() => HostCommandLine.Parse(args));
}
